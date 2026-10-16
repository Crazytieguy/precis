import { atom, read, update } from 'claude-code'
import type { EngineInterface, On, PluginState } from 'claude-code'

import { noticeRows } from './notice-band'

// Moving an install onto the precis plugin built for this machine, offered as
// a row in the band above the prompt. Both plugins carry this file;
// plugin_shared_files_match keeps the copies identical.

/** A plugin to put in another's place, and the row that offers it. */
export type Switch = Omit<NonNullable<PluginState['precis']['offer']>, 'busy'>

const offer = atom({ plugin: 'precis', key: 'offer' } as const, null)
const dismissed = atom({ plugin: 'precis', key: 'dismissed' } as const, false)

/** The target triple the `precis-<target>` plugins are named by, or undefined off macOS and Linux on x86_64 or arm64. */
export async function platformTarget($: EngineInterface): Promise<string | undefined> {
  const { stdout } = await $.process.run(['uname', '-sm']).catch(() => ({ stdout: '' }))
  const [os, arch] = stdout.trim().split(/\s+/)
  const osPart = os === 'Darwin' ? 'apple-darwin' : os === 'Linux' ? 'unknown-linux-gnu' : undefined
  const archPart = arch === 'x86_64' ? 'x86_64' : arch === 'arm64' || arch === 'aarch64' ? 'aarch64' : undefined
  return osPart && archPart ? `${archPart}-${osPart}` : undefined
}

/** Puts `found` in the band, or takes the row down when there is none. */
export async function refreshOffer($: EngineInterface, found: Switch | undefined): Promise<void> {
  await update($, offer, () => found ?? null)
}

/** Draws the offered switch as a band row whose button makes it. */
export function drawOffer(on: On): void {
  on('ui.render', { component: 'AbovePrompt' }, async ($, e, next) => {
    const [below, current, isDismissed] = await Promise.all([next(e), read($, offer), read($, dismissed)])
    if (current === null || isDismissed) return below

    const ui = $.ui.resolve(e)
    const notice = {
      id: 'switch',
      severity: current.severity,
      text: current.text,
      actions: [{ id: 'switch', label: current.label, kind: 'plugin' as const }],
      busy: current.busy,
    }
    const rows = noticeRows(ui, 'precis', [notice], {
      onAction: () => makeSwitch($, current),
      onDismiss: () => update($, dismissed, () => true),
    })
    return (
      <ui.Box flexDirection="column">
        {rows}
        {below}
      </ui.Box>
    )
  })
}

async function makeSwitch($: EngineInterface, current: Switch) {
  const setBusy = (busy: string | undefined) =>
    update($, offer, value => (value === null ? value : { ...value, busy }))
  await setBusy('switching…')
  const error = await switchPlugin($, current.from, current.to).catch(String)
  if (error !== undefined) {
    await setBusy(undefined)
    $.ui.toast(`Switch failed: ${error}`)
    return
  }
  // Both plugins are named precis and so share this state: the row goes
  // before the reload hands it to the plugin just installed.
  await update($, offer, () => null)
  $.ui.toast(`Switched to ${current.to.replace(/@precis$/, '')}`)
  await $.command.run({ command: 'reload-plugins' })
}

/**
 * Installs `to` where `from` is enabled and takes `from` out there.
 * Resolves the error text when a step fails.
 *
 * A platform's plugin in a checked-in project file would break teammates on
 * other machines, so a project that enables `from` gets `to` installed and
 * `from` disabled in local settings, its own file untouched.
 */
async function switchPlugin($: EngineInterface, from: string, to: string): Promise<string | undefined> {
  const sources = ['user', 'project', 'local'] as const
  const settings = await Promise.all(sources.map(source => $.settings.read({ source })))
  const enabledIn = sources.filter((_, index) => {
    const enabled = settings[index]!.enabledPlugins as Record<string, unknown> | undefined
    return enabled?.[from] === true
  })
  const scope = enabledIn.includes('user') ? 'user' : 'local'
  // A local uninstall clears the local entry a disable writes, so the
  // disable comes last.
  const steps = [
    ['install', to, '--scope', scope],
    ...enabledIn.filter(source => source !== 'project').map(source => ['uninstall', from, '--scope', source]),
    ...(enabledIn.includes('project') ? [['disable', from, '--scope', 'local']] : []),
  ]
  for (const step of steps) {
    const { exitCode, stdout, stderr } = await $.process.run(['claude', 'plugin', ...step], { timeoutMs: 120_000 })
    if (exitCode !== 0) return (stderr.trim() || stdout.trim() || `claude plugin ${step[0]} failed`).split('\n')[0]
  }
  return undefined
}
