const invoke = window.__TAURI__.invoke;

// When emitting events, avail: `listen` and `once`
// const { emit } = window.__TAURI__.event;

export async function hideWindow() {
  invoke('hide_window');
}
