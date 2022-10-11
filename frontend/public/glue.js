const invoke = window.__TAURI__.invoke;
const { emit, listen } = window.__TAURI__.event;

export function eventListener() {
  return listen;
}

export async function listenToEvent(evtName, cb) {
  // return 'Hello FROM LISTENER';
  return listen(evtName, (event) => {
    const { payload } = event;
    console.log('CALLING Callback with data:', payload);
    cb(payload);
    // return event.payload;
  });
  // return await listen(evtName, (event) => {
  //   console.log('Got EVENT: ', event.payload);
  //   cb(event);
  // });
}
// // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
// const unlisten = await listen('timer_started', (event) => {
//   // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
//   // event.payload is the payload object
//   console.log('EVENT PAYLOAD', event.payload);
// });

// --------------
// --- App ------
export async function hideMainWindow() {
  invoke('hide_main_window');
}
export async function showMainWindow() {
  invoke('show_main_window');
}
export async function quitApp() {
  invoke('quit_app');
}

// ----------------
// --- Timer ------
export async function timerStart() {
  invoke('timer_test');
}
