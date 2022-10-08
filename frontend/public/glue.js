const invoke = window.__TAURI__.invoke;

// When emitting events, avail: `listen` and `once`
// const { emit } = window.__TAURI__.event;

export async function hideMainWindow() {
  invoke('hide_main_window');
}
export async function showMainWindow() {
  invoke('show_main_window');
}
export async function quitApp() {
  invoke('quit_app');
}
