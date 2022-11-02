import { invoke } from '@tauri-apps/api/tauri';

// --------------
// --- App ------
export async function closeMainWindow() {
  return await invoke('close_main_window');
}
export async function showMainWindow() {
  return await invoke('show_main_window');
}
export async function quitApp() {
  return invoke('quit_app');
}

// ----------------
// --- Timer ------
export async function timerStart(): Promise<void> {
  return invoke('timer_start');
}
export async function timerPause(): Promise<TimerState> {
  return invoke('timer_pause');
}
export async function timerGetState(): Promise<TimerState> {
  return invoke('timer_get_state');
}
export async function timerReset(): Promise<TimerState> {
  return invoke('timer_reset');
}
// --- Timer finished ---
export async function timerFinishedStartNew(): Promise<TimerState> {
  return invoke('timer_finished_start_new');
}
export async function timerFinishedCloseWindow(): Promise<void> {
  return invoke('timer_finished_close_window');
}

// ------------------
// --- Types --------
export enum TimerStatus {
  Idle = 'Idle',
  Running = 'Running',
  Paused = 'Paused',
  Finished = 'Finished',
}

export interface TimerState {
  status: TimerStatus;
  minutes: number;
  seconds: number;
}
