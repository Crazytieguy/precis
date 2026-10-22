import type { EngineInterface, Register } from 'claude-code'

import { drawOffer, platformTarget, refreshOffer } from './switch'

const TOOL = 'mcp__precis__summarize'

/** What the engine answers a call of a plugin's tool that its permission check let through. */
const UNSERVED = 'no tool.call hook answered this call'

/** The tool's arguments, as the model gives them and as the permission check reads them. */
type Args = { path?: string; token_budget?: number }

/** This machine's target, and the binary bundle this plugin carries, if any. */
type Platform = { target: string | undefined; bundle: { target: string; path: string } | undefined }

/** What the session needs once: where it runs, and the binary that runs there. */
type Setup = { platform: Promise<Platform>; binary: Promise<string | undefined> }

export const register: Register = (on, options) => {
  const tokenBudget = Number(options.token_budget)
  let setup: Setup | undefined
  // The input each call's permission check read, by `tool_use_id`: a
  // PreToolUse hook's rewrite shows here and not on the call's own `e`.
  const checked = new Map<string, Args>()

  drawOffer(on)

  on('session.start', async ($, e, next) => {
    const bin = await (setup ??= setUp($)).binary
    if (bin !== undefined) {
      await $.tool.register({
        name: 'summarize',
        description: await about($, bin),
        inputSchema: {
          type: 'object',
          properties: {
            path: {
              type: 'string',
              description: 'Directory or file to summarize. Default: the working directory.',
            },
            token_budget: {
              type: 'integer',
              minimum: 1,
              description: `Output budget in o200k_base tokens. Default: ${tokenBudget}.`,
            },
          },
        },
        isDeferred: false,
      })
    }
    return next(e)
  })

  on('tool.check', { tool: TOOL }, ($, e, next) => {
    if (e.tool_use_id !== undefined) checked.set(e.tool_use_id, e.input as Args)
    return next(e)
  })

  on('tool.call', { tool: TOOL }, async ($, e, next) => {
    const args = e as Args
    const bin = await (setup ??= setUp($)).binary
    if (bin === undefined) return { deny: 'precis has no binary to run' }
    const path = await expandHome($, args.path ?? '.')
    const read = await readDecision($, path)
    if ('deny' in read) return read
    if (read.decision === 'ask') {
      const decided = await next(e)
      const input = checked.get(e.tool_use_id)
      checked.delete(e.tool_use_id)
      if (decided.isError !== true || !decided.text?.includes(UNSERVED)) return decided
      if (input === undefined || input.path !== args.path || input.token_budget !== args.token_budget) {
        return { deny: 'A hook changed the path or budget before the permission check; precis runs only what it checked' }
      }
    }
    const run = await $.process.run(
      [bin, '--token-budget', String(args.token_budget ?? tokenBudget), '--', path],
      { timeoutMs: 120_000 },
    )
    if (run.exitCode !== 0) return { deny: run.stderr.trim() }
    return { result: run.stdout === '' ? run.stderr.trim() : run.stdout }
  })

  // The main conversation's context, not subagents'. A compacted conversation
  // keeps what it needs of the overview in its summary, and a resumed one
  // still holds it.
  on('classic.SessionStart', async ($, e, next) => {
    if (e.source !== 'startup' && e.source !== 'clear') return next(e)
    const { platform, binary } = (setup ??= setUp($))
    const [below, overview] = await Promise.all([
      next(e),
      workingDirOverview($, binary, tokenBudget),
      offerFix($, platform),
    ])
    if (overview === undefined) return below
    return { ...below, additionalContext: [...(below.additionalContext ?? []), overview] }
  })
}

function setUp($: EngineInterface): Setup {
  const platform = detectPlatform($)
  return { platform, binary: bundledBinary($, platform) }
}

async function workingDirOverview(
  $: EngineInterface,
  binary: Promise<string | undefined>,
  tokenBudget: number,
): Promise<string | undefined> {
  const bin = await binary
  if (bin === undefined) return undefined
  const run = await $.process
    .run([bin, '--token-budget', String(tokenBudget), '.'], { timeoutMs: 60_000 })
    .catch((error: unknown) => {
      $.ui.log(`precis: ${String(error)}`)
      return undefined
    })
  if (run === undefined) return undefined
  if (run.exitCode !== 0) {
    $.ui.log(`precis: ${run.stderr.trim()}`)
    return undefined
  }
  if (run.stdout === '') return undefined
  const fence = '`'.repeat(Math.max(3, longestBacktickRun(run.stdout) + 1))
  return `## precis\n\nOutput of \`${TOOL}\` on the working directory:\n\n${fence}\n${run.stdout.trimEnd()}\n${fence}`
}

/**
 * The precis binary: `PRECIS_DEV` when set, for a plugin loaded from source,
 * which bundles none; else the one this plugin bundles for this machine,
 * unpacked on first use. A bundle for another machine is the switch row's to
 * report.
 */
async function bundledBinary($: EngineInterface, platform: Promise<Platform>): Promise<string | undefined> {
  const dev = await $.env.get('PRECIS_DEV')
  if (dev !== undefined) return dev
  const { target, bundle } = await platform
  if (target === undefined) {
    $.ui.log('precis: no plugin build for this machine; install the standalone CLI')
    return undefined
  }
  if (bundle === undefined) {
    $.ui.log('precis: no binary bundled; set PRECIS_DEV to run the plugin from source')
    return undefined
  }
  if (bundle.target !== target) return undefined
  const run = await $.process.run(['bash', `${$.plugin.root}/scripts/unpack.sh`, bundle.path])
  if (run.exitCode === 0) return run.stdout.trim()
  $.ui.log(`precis: ${run.stderr.trim()}`)
  return undefined
}

async function detectPlatform($: EngineInterface): Promise<Platform> {
  const bin = `${$.plugin.root}/bin`
  const [target, entries] = await Promise.all([platformTarget($), $.fs.list(bin).catch(() => [])])
  for (const { name } of entries) {
    const bundled = name.match(/^precis-(.+)\.tar\.xz$/)
    if (bundled) return { target, bundle: { target: bundled[1]!, path: `${bin}/${name}` } }
  }
  return { target, bundle: undefined }
}

/** Offers this machine's plugin in place of one bundling another's binary. */
async function offerFix($: EngineInterface, platform: Promise<Platform>): Promise<void> {
  const { target, bundle } = await platform
  const isWrong = target !== undefined && bundle !== undefined && bundle.target !== target
  await refreshOffer(
    $,
    isWrong
      ? {
          severity: 'problem',
          text: 'wrong platform variant installed',
          label: 'Fix',
          from: `precis-${bundle.target}@precis`,
          to: `precis-${target}@precis`,
        }
      : undefined,
  )
}

/** The part of `precis --help` above its usage line: what it does and the notation. */
async function about($: EngineInterface, bin: string): Promise<string> {
  const { stdout } = await $.process.run([bin, '--help'])
  return stdout.split('\n\nUsage:')[0]!.trim()
}

async function expandHome($: EngineInterface, path: string): Promise<string> {
  if (path !== '~' && !path.startsWith('~/')) return path
  return `${await $.env.get('HOME')}${path.slice(1)}`
}

/**
 * Read's decision on `path`, or why precis may not read it. A plugin's own
 * tool reaches the permission check only through `next`, which ends at the
 * engine's error for a call no hook served, so the tool follows Read's
 * decision on the same path and puts only Read's ask to the permission mode:
 * the dialog, the auto-mode classifier, a headless host's refusal. Plan mode
 * refuses every call that reaches it, so an allow skips it.
 */
async function readDecision(
  $: EngineInterface,
  path: string,
): Promise<{ decision: 'allow' | 'ask' } | { deny: string }> {
  const stat = await $.fs.stat(path, { resolve: true }).catch(() => undefined)
  if (stat === undefined) return { deny: `${path} does not exist` }
  const target = stat.realPath ?? path
  const { decision, reason } = await $.tool.check({ tool: 'Read', input: { file_path: target } })
  if (decision === 'deny') return { deny: reason ?? `Reading ${target} is denied` }
  return { decision }
}

function longestBacktickRun(text: string): number {
  return Math.max(0, ...(text.match(/`+/g) ?? []).map(run => run.length))
}
