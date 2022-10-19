import type { NextComponentType } from 'next';
import { timerStart } from '../services';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

let actionBtnClasses =
  'rounded-md w-1/3 p-2 border text-2xl hover:bg-white hover:text-neutral-500';

interface TimerState {
  minutes: number;
  seconds: number;
  status: 'idle' | 'running' | 'paused';
}

const getTimerState = (): TimerState => {
  return {
    minutes: 25,
    seconds: 0,
    status: 'idle',
  };
};

const Timer: NextComponentType = () => {
  const [timerState, setTimerState] = useState<TimerState>(getTimerState());

  const listenTimerStateChanged = async () => {
    const unlisten = await listen('timer_state_changed', (event) => {
      console.log('RECEIVED EVENT', event.payload);
    });
    return unlisten;
  };

  useEffect(() => {
    listenTimerStateChanged();
  }, []);

  const handleStartBtnClick = async () => {
    console.log('start btn clickeded');
    await timerStart();
  };

  const renderActionButtons = () => {
    return (
      <button onClick={handleStartBtnClick} className={actionBtnClasses}>
        {'Start'}
      </button>
    );
  };

  return (
    <div className='flex grow flex-col justify-center items-center text-center'>
      <div className='text-9xl'>
        <span>{padWithZero(timerState.minutes)}</span>
        {''} : {''}
        <span>{padWithZero(timerState.seconds)}</span>
      </div>

      {/* Divider */}
      <div className='my-10'></div>

      <div className='w-full'>{renderActionButtons()}</div>
    </div>
  );
};

export default Timer;

const padWithZero = (val: number): string => {
  const valString = val.toString();
  if (valString.length < 2) {
    return '0' + valString;
  }
  return valString;
};
