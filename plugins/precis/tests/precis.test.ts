import type { On, ProcessRunResult, ToolCheckDecision } from 'claude-code'
import { expect, mock, test } from 'claude-code/testing'

const SUMMARY = '.cargo/\n  config.toml\nREADME.md\n  1→# demo\n  2→```sh\n'
const HELP = 'Summarize a directory or file within a token budget.\n\n`N→` rows are source line N.\n\nUsage: precis [OPTIONS] [PATH]\n'
const BINARY = '/cache/precis/v0.3.0/precis'

const BAND = {
  plugin: 'precis',
  component: 'AbovePrompt' as const,
  props: { hasSurvey: false, isCollapsed: false, isWorking: false, maxRows: 20, bodyColumns: 140, scroll: { offset: 0, bodyRows: 19 }, view: {} },
}

type World = {
  bundled?: string
  dev?: string
  readDecision?: ToolCheckDecision
  granted?: boolean
  turns?: number
  refuseTools?: boolean
}

function ran(stdout: string): { value: ProcessRunResult } {
  return { value: { exitCode: 0, stdout, stderr: '', isStdoutTruncated: false, isStderrTruncated: false } }
}

/**
 * The host beneath the plugin: an arm64 Mac, the binary the plugin bundles,
 * Read's verdict, the permission mode's, the prompts the session already
 * holds. Returns what the plugin ran, registered and invoked, and the context
 * each prompt entered with.
 */
function world(on: On, w: World = {}) {
  const runs: (readonly string[])[] = []
  const registered: string[] = []
  const commands: string[] = []
  const prompts: (readonly string[])[] = []
  mock.env(on, { HOME: '/home/u', ...(w.dev === undefined ? {} : { PRECIS_DEV: w.dev }) })
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('session.end', ($, e) => ({ sessionId: e.sessionId }))
  on('session.turns', () => ({ value: w.turns ?? 0 }))
  on('prompt.submit', ($, e) => {
    if (e.text === 'blocked') return { drop: 'a hook beneath refused it' }
    prompts.push(e.context ?? [])
    return { text: e.text, context: e.context }
  })
  on('fs.list', () => ({
    value: [{ name: `precis-${w.bundled ?? 'aarch64-apple-darwin'}.tar.xz`, kind: 'file', size: 1, mtimeMs: 0, isLink: false }],
  }))
  on('fs.stat', ($, e) => {
    if (e.path.includes('missing')) return { deny: 'ENOENT' }
    const realPath = e.path.startsWith('/') ? e.path : `/repo/${e.path}`
    return { value: { kind: 'dir', size: 0, mtimeMs: 0, isLink: false, realPath } }
  })
  on('settings.read', ($, e) => ({
    value: e.source === 'user' ? { enabledPlugins: { [`precis-${w.bundled}@precis`]: true } } : {},
  }))
  on('tool.check', { tool: 'Read' }, () => ({ decision: w.readDecision ?? 'allow' }))
  on('tool.call', { tool: 'mcp__precis__summarize' }, () => {
    const text = w.granted
      ? 'precis registered the tool summarize but no tool.call hook answered this call'
      : "Claude requested permissions to use mcp__precis__summarize, but you haven't granted it yet."
    return { result: `Error: ${text}`, text, isError: true }
  })
  on('command.run', ($, e) => {
    commands.push(e.command)
    return { text: '' }
  })
  on('ui.log', () => ({ value: undefined }))
  on('ui.render', () => ({ type: 'Box', props: {}, children: [] }) as never)
  on('tool.register', ($, e) => {
    if (w.refuseTools) return { deny: 'refused by policy' }
    registered.push(e.name)
    return { value: { tool: `mcp__precis__${e.name}` } }
  })
  on('process.run', ($, e) => {
    runs.push(e.argv)
    if (e.argv[0] === 'uname') return ran('Darwin arm64\n')
    if (e.argv[0] === 'bash') return ran(`${BINARY}\n`)
    if (e.argv[0] === 'claude') return ran('')
    return ran(e.argv.includes('--help') ? HELP : SUMMARY)
  })
  return { runs, registered, commands, prompts }
}

const start = { cwd: '/repo', surface: null, isInteractive: false } as const
const prompt = (text: string) => ({ text, wait: false, origin: { kind: 'composer' } }) as const
const overviewRuns = (runs: (readonly string[])[]) => runs.filter(argv => argv.at(-1) === '.')

test("a new conversation's first prompt carries the summary of the working directory", async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  await $.prompt.submit(prompt('and then'))
  const [first, second] = w.prompts
  expect(first).toHaveLength(1)
  expect(first![0]).toContain('Output of `mcp__precis__summarize` on the working directory:')
  expect(first![0]).toContain('\n````\n.cargo/\n')
  expect(second).toEqual([])
  expect(overviewRuns(w.runs)).toEqual([[BINARY, '--token-budget', '3000', '.']])
})

test("a cleared conversation's first prompt carries a fresh summary", async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  await $.session.end({ reason: 'clear', sessionId: 's1', resume: { id: 's1' } })
  await $.prompt.submit(prompt('fresh start'))
  expect(w.prompts[1]).toHaveLength(1)
  expect(w.prompts[1]![0]).toContain('Output of `mcp__precis__summarize` on the working directory:')
  expect(overviewRuns(w.runs)).toHaveLength(2)
})

test('a resumed conversation, or a plugin reloaded mid-conversation, gets no new summary', async ($, on) => {
  const w = world(on, { turns: 3 })
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  expect(w.prompts).toEqual([[]])
  expect(overviewRuns(w.runs)).toHaveLength(0)
})

test('a conversation resumed in the same process gets no summary', async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  await $.session.end({ reason: 'resume', sessionId: 's1', resume: { id: 's1' } })
  await $.prompt.submit(prompt('hi'))
  await $.session.end({ reason: 'clear', sessionId: 's2', resume: { id: 's2' } })
  await $.session.end({ reason: 'resume', sessionId: 's3', resume: { id: 's3' } })
  await $.prompt.submit(prompt('and then'))
  expect(w.prompts).toEqual([[], []])
  expect(overviewRuns(w.runs)).toHaveLength(0)
})

test('a slash command or a refused prompt leaves the summary for the next prompt', async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  await $.prompt.submit({ ...prompt('/model'), command: { name: 'model', args: '' } })
  await $.prompt.submit(prompt('blocked'))
  await $.prompt.submit(prompt('hi'))
  expect(w.prompts).toHaveLength(2)
  expect(w.prompts[1]![0]).toContain('Output of `mcp__precis__summarize` on the working directory:')
})

test('the token budget option sets the summary budget', { options: { token_budget: 8000 } }, async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  expect(w.runs).toContainEqual([BINARY, '--token-budget', '8000', '.'])
})

test('where policy refuses the tool, the summary still comes and names precis itself', async ($, on) => {
  const w = world(on, { refuseTools: true })
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  expect(w.registered).toEqual([])
  expect(w.prompts[0]![0]).toContain('Output of `precis` on the working directory:')
})

test('the binary is the one bundled for this machine', async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  expect(w.runs).toContainEqual(['bash', expect.stringMatching(/\/scripts\/unpack\.sh$/), expect.stringMatching(/\/bin\/precis-aarch64-apple-darwin\.tar\.xz$/)])
  expect(w.registered).toEqual(['summarize'])
})

test('PRECIS_DEV names the binary of a plugin loaded from source', async ($, on) => {
  const w = world(on, { dev: '/src/target/release/precis' })
  await $.session.start(start)
  expect(w.runs).toContainEqual(['/src/target/release/precis', '--help'])
  expect(w.runs.some(argv => argv[0] === 'bash')).toBe(false)
})

test("a bundle for another machine offers this machine's plugin instead of running", async ($, on) => {
  const w = world(on, { bundled: 'x86_64-unknown-linux-gnu' })
  await $.session.start(start)
  await $.prompt.submit(prompt('hi'))
  expect(w.prompts).toEqual([[]])
  expect(w.registered).toEqual([])

  const ui = await $.ui.mount({ ...BAND, surface: 'terminal' })
  expect(await ui.find({ type: 'Text', text: /wrong platform variant installed/ })).toBeDefined()
  await ui.press({ key: 'precis:switch:switch' })
  const claude = w.runs.filter(argv => argv[0] === 'claude').map(argv => argv.slice(2))
  expect(claude).toEqual([
    ['install', 'precis-aarch64-apple-darwin@precis', '--scope', 'user'],
    ['uninstall', 'precis-x86_64-unknown-linux-gnu@precis', '--scope', 'user'],
  ])
  expect(w.commands).toEqual(['reload-plugins'])
  await ui.unmount()
})

test('the tool runs precis on a path the user lets Read see', async ($, on) => {
  const w = world(on)
  await $.session.start(start)
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: 'src', token_budget: 500 })
  expect(result.result).toBe(SUMMARY)
  expect(w.runs).toContainEqual([BINARY, '--token-budget', '500', '--', 'src'])
})

test('a path Read would ask about runs once the permission mode lets the call through', async ($, on) => {
  const w = world(on, { readDecision: 'ask', granted: true })
  await $.session.start(start)
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: '~/notes' })
  expect(result.result).toBe(SUMMARY)
  expect(w.runs).toContainEqual([BINARY, '--token-budget', '3000', '--', '/home/u/notes'])
})

test('a path Read would ask about is refused as the permission mode refuses it', async ($, on) => {
  const w = world(on, { readDecision: 'ask' })
  await $.session.start(start)
  const before = w.runs.length
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: '/etc' })
  expect(result.isError).toBe(true)
  expect(result.text).toBe("Claude requested permissions to use mcp__precis__summarize, but you haven't granted it yet.")
  expect(w.runs).toHaveLength(before)
})

test('a path a PreToolUse hook rewrote on its way to the permission check is refused', async ($, on) => {
  const w = world(on, { readDecision: 'ask', granted: true })
  on('classic.PreToolUse', () => ({ updatedInput: { path: '/home/u' } }))
  await $.session.start(start)
  const before = w.runs.length
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: '/etc' })
  expect(result.deny).toBe('A hook changed the path or budget before the permission check; precis runs only what it checked')
  expect(w.runs).toHaveLength(before)
})

test('a path Read is denied is refused without running precis', async ($, on) => {
  const w = world(on, { readDecision: 'deny' })
  await $.session.start(start)
  const before = w.runs.length
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: 'secrets' })
  expect(result.deny).toMatch(/^Reading \/.*\/secrets is denied$/)
  expect(w.runs).toHaveLength(before)
})

test('a missing path is refused', async ($, on) => {
  world(on)
  await $.session.start(start)
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: 'missing' })
  expect(result.deny).toBe('missing does not exist')
})
