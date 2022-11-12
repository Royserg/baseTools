import {
  timerStart,
  timerGetState,
  TimerState,
  TimerStatus,
  timerPause,
  timerReset,
  closeSunGlassWindow,
  openSunGlassWindow,
} from '../services';
import { listen } from '@tauri-apps/api/event';
import { ComponentType, useEffect, useState } from 'react';
import { VscDebugStart, VscDebugPause, VscDebugStop } from 'react-icons/vsc';

interface SunGlassProps {}

export const SunGlass: ComponentType<SunGlassProps> = () => {
  const [isActive, setIsActive] = useState<boolean>(false);

  const onSwitchClick = async () => {
    setIsActive((isActive) => !isActive);
    if (isActive) {
      await closeSunGlassWindow();
    }
    await openSunGlassWindow();
  };

  return (
    <div className='flex grow flex-col justify-center items-center text-center'>
      <button className='border rounded-xl w-3/4' onClick={onSwitchClick}>
        {isActive ? 'off' : 'on'}
      </button>
    </div>
  );
};
