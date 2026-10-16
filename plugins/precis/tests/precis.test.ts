import type { On, ProcessRunResult, ToolCheckDecision } from 'claude-code'
import { expect, mock, test } from 'claude-code/testing'

const SUMMARY = '.cargo/\n  config.toml\nREADME.md\n  1→# demo\n  2→```sh\n'
const HELP = 'Summarize a directory or file within a token budget.\n\n`N→` rows are source line N.\n\nUsage: precis [OPTIONS] [PATH]\n'
const BINARY = '/cache/precis/v0.3.0/precis'

const BAND = {
  plugin: 'precis',
  component: 'AbovePrompt' as const,
  props: { hasSurvey: false, isWorking: false, maxRows: 20, bodyColumns: 140, scroll: { offset: 0, bodyRows: 19 }, view: {} },
}

type World = {
  bundled?: string
  dev?: string
  readDecision?: ToolCheckDecision
  answer?: string
}

function ran(stdout: string): { value: ProcessRunResult } {
  return { value: { exitCode: 0, stdout, stderr: '', isStdoutTruncated: false, isStderrTruncated: false } }
}

/**
 * The host beneath the plugin: an arm64 Mac, the binary the plugin bundles,
 * Read's verdict, the user. Returns what the plugin ran, asked, registered
 * and invoked.
 */
function world(on: On, w: World = {}) {
  const runs: (readonly string[])[] = []
  const asked: string[] = []
  const registered: string[] = []
  const commands: string[] = []
  mock.env(on, { HOME: '/home/u', ...(w.dev === undefined ? {} : { PRECIS_DEV: w.dev }) })
  on('session.start', ($, e) => ({ cwd: e.cwd }))
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
  on('tool.check', () => ({ decision: w.readDecision ?? 'allow' }))
  on('tool.call', { tool: 'AskUserQuestion' }, ($, e) => {
    const question = e.questions[0]!.question
    asked.push(question)
    if (w.answer === undefined) return { deny: 'no one to ask' }
    return { result: { questions: e.questions, answers: { [question]: w.answer } } }
  })
  on('command.run', ($, e) => {
    commands.push(e.command)
    return { text: '' }
  })
  on('ui.log', () => ({ value: undefined }))
  on('ui.render', () => ({ type: 'Box', props: {}, children: [] }) as never)
  on('tool.register', ($, e) => {
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
  return { runs, asked, registered, commands }
}

const start = { cwd: '/repo', surface: null, isInteractive: false } as const

test('a new or cleared conversation starts with the summary of the working directory', async ($, on) => {
  const w = world(on)
  on('classic.SessionStart', () => ({ additionalContext: ['another hook'] }))
  for (const source of ['startup', 'clear'] as const) {
    const { additionalContext } = await $.classic.SessionStart({ source })
    expect(additionalContext).toHaveLength(2)
    expect(additionalContext![0]).toBe('another hook')
    expect(additionalContext![1]).toContain('Output of `mcp__precis__summarize` on the working directory:')
    expect(additionalContext![1]).toContain('\n````\n.cargo/\n')
  }
  expect(w.runs).toContainEqual([BINARY, '--token-budget', '3000', '.'])
})

test('a compacted or resumed conversation gets no new summary', async ($, on) => {
  const w = world(on)
  on('classic.SessionStart', () => ({ additionalContext: ['another hook'] }))
  for (const source of ['compact', 'resume'] as const) {
    const { additionalContext } = await $.classic.SessionStart({ source })
    expect(additionalContext).toEqual(['another hook'])
  }
  expect(w.runs).toHaveLength(0)
})

test('the token budget option sets the summary budget', { options: { token_budget: 8000 } }, async ($, on) => {
  const w = world(on)
  on('classic.SessionStart', () => ({}))
  await $.classic.SessionStart({ source: 'startup' })
  expect(w.runs).toContainEqual([BINARY, '--token-budget', '8000', '.'])
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
  on('classic.SessionStart', () => ({}))
  const { additionalContext } = await $.classic.SessionStart({ source: 'startup' })
  await $.session.start(start)
  expect(additionalContext).toBeUndefined()
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

test('a path Read would ask about is put to the user', async ($, on) => {
  const w = world(on, { readDecision: 'ask', answer: 'No' })
  await $.session.start(start)
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: '/etc' })
  expect(result.deny).toBe('The user declined to let precis read /etc')
  expect(w.asked).toHaveLength(1)
})

test('with nobody to ask, a path Read would ask about is refused', async ($, on) => {
  world(on, { readDecision: 'ask' })
  await $.session.start(start)
  const result = await $.tool.call({ tool: 'mcp__precis__summarize', path: '~/notes' })
  expect(result.deny).toBe("Reading /home/u/notes needs the user's permission, and they could not be asked")
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
