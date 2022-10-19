import { invoke } from '@tauri-apps/api/tauri';

// export async function listenToEvent(evtName, cb) {
//   // return 'Hello FROM LISTENER';
//   return listen(evtName, (event) => {
//     const { payload } = event;
//     console.log('CALLING Callback with data:', payload);
//     cb(payload);
//     // return event.payload;
//   });
// }

// --------------
// --- App ------
export async function hideMainWindow() {
  return await invoke('hide_main_window');
}
export async function showMainWindow() {
  return await invoke('show_main_window');
}
export async function quitApp() {
  return invoke('quit_app');
}

// ----------------
// --- Timer ------
export async function timerStart() {
  return invoke('timer_start');
}
