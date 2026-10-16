import type { On, ProcessRunResult, SettingsSource } from 'claude-code'
import { expect, test } from 'claude-code/testing'

const BAND = {
  plugin: 'precis',
  component: 'AbovePrompt' as const,
  props: { hasSurvey: false, isWorking: false, maxRows: 20, bodyColumns: 140, scroll: { offset: 0, bodyRows: 19 }, view: {} },
}
const SWITCH = { key: 'precis:switch:switch' }

type Host = { machine?: string; enabledIn?: SettingsSource[]; failing?: string }

function ran(exitCode: number, stdout: string, stderr = ''): { value: ProcessRunResult } {
  return { value: { exitCode, stdout, stderr, isStdoutTruncated: false, isStderrTruncated: false } }
}

/** The host beneath the plugin: its machine, where `precis@precis` is enabled, and a `claude plugin` step that fails. */
function host(on: On, h: Host = {}) {
  const claude: string[][] = []
  const commands: string[] = []
  const toasts: string[] = []
  on('classic.SessionStart', () => ({}))
  on('settings.read', ($, e) => ({
    value: (h.enabledIn ?? ['user']).includes(e.source!) ? { enabledPlugins: { 'precis@precis': true } } : {},
  }))
  on('process.run', ($, e) => {
    if (e.argv[0] === 'uname') return ran(0, `${h.machine ?? 'Darwin arm64'}\n`)
    claude.push(e.argv.slice(2))
    return e.argv[2] === h.failing ? ran(1, '', `${h.failing} failed: archive not found\nmore`) : ran(0, '')
  })
  on('command.run', ($, e) => {
    commands.push(e.command)
    return { text: '' }
  })
  on('ui.toast', ($, e) => {
    toasts.push(e.text)
    return { value: undefined } as never
  })
  on('ui.render', () => ({ type: 'Box', props: {}, children: [] }) as never)
  return { claude, commands, toasts }
}

test("the band offers this machine's plugin in place of the plain one", async ($, on) => {
  const { claude, commands, toasts } = host(on)
  await $.classic.SessionStart({ source: 'startup' })
  const ui = await $.ui.mount({ ...BAND, surface: 'terminal' })
  expect(await ui.find({ type: 'Text', text: /superseded by precis-aarch64-apple-darwin/ })).toBeDefined()

  await ui.press(SWITCH)
  expect(claude).toEqual([
    ['install', 'precis-aarch64-apple-darwin@precis', '--scope', 'user'],
    ['uninstall', 'precis@precis', '--scope', 'user'],
  ])
  expect(commands).toEqual(['reload-plugins'])
  expect(toasts).toEqual(['Switched to precis-aarch64-apple-darwin'])
  expect(await ui.find({ key: SWITCH.key })).toBeUndefined()
  await ui.unmount()
})

test('a project that enables the plain plugin keeps its file; the switch goes in local settings', async ($, on) => {
  const { claude } = host(on, { machine: 'Linux x86_64', enabledIn: ['project'] })
  await $.classic.SessionStart({ source: 'startup' })
  const ui = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await ui.press(SWITCH)
  expect(claude).toEqual([
    ['install', 'precis-x86_64-unknown-linux-gnu@precis', '--scope', 'local'],
    ['disable', 'precis@precis', '--scope', 'local'],
  ])
  await ui.unmount()
})

test('enabled in the project and installed locally, the plain plugin is left disabled locally', async ($, on) => {
  const { claude } = host(on, { enabledIn: ['project', 'local'] })
  await $.classic.SessionStart({ source: 'startup' })
  const ui = await $.ui.mount({ ...BAND, surface: 'terminal' })
  await ui.press(SWITCH)
  expect(claude).toEqual([
    ['install', 'precis-aarch64-apple-darwin@precis', '--scope', 'local'],
    ['uninstall', 'precis@precis', '--scope', 'local'],
    ['disable', 'precis@precis', '--scope', 'local'],
  ])
  await ui.unmount()
})

test('a failed switch says why and leaves the plain plugin and the button', async ($, on) => {
  const { claude, commands, toasts } = host(on, { failing: 'install' })
  await $.classic.SessionStart({ source: 'startup' })
  const ui = await $.ui.mount({ ...BAND, surface: 'terminal' })
  await ui.press(SWITCH)
  expect(claude).toHaveLength(1)
  expect(commands).toEqual([])
  expect(toasts).toEqual(['Switch failed: install failed: archive not found'])
  expect(await ui.find({ key: SWITCH.key })).toBeDefined()
  await ui.unmount()
})

test('off the platforms precis builds for, nothing is offered', async ($, on) => {
  host(on, { machine: 'Linux riscv64' })
  await $.classic.SessionStart({ source: 'startup' })
  const ui = await $.ui.mount({ ...BAND, surface: 'terminal' })
  expect(await ui.find({ key: SWITCH.key })).toBeUndefined()
  await ui.unmount()
})
