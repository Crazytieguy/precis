declare module 'claude-code' {
  interface PluginState {
    precis: {
      // The switch hooks/switch.tsx offers in the band, with what its button
      // is doing while it runs; null when there is none.
      offer: {
        severity: 'urgent' | 'problem' | 'action' | 'info'
        text: string
        label: string
        from: string
        to: string
        busy?: string
      } | null
      dismissed: boolean
    }
  }
}
