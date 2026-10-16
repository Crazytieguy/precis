import type { EngineInterface, Register } from 'claude-code'

import { drawOffer, platformTarget, refreshOffer } from './switch'

// The plain `precis` plugin: the shell hooks beside this module keep working,
// and the band offers the plugin built for this machine in their place.

export const register: Register = on => {
  on('classic.SessionStart', async ($, e, next) => {
    if (e.source !== 'startup' && e.source !== 'clear') return next(e)
    const [below] = await Promise.all([next(e), offerReplacement($)])
    return below
  })
  drawOffer(on)
}

async function offerReplacement($: EngineInterface): Promise<void> {
  const target = await platformTarget($)
  await refreshOffer(
    $,
    target === undefined
      ? undefined
      : {
          severity: 'action',
          text: `superseded by precis-${target}`,
          label: 'Switch',
          from: 'precis@precis',
          to: `precis-${target}@precis`,
        },
  )
}
