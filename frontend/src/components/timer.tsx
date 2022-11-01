import {
  timerStart,
  timerGetState,
  TimerState,
  TimerStatus,
  timerPause,
  timerReset,
} from '../services';
import { listen } from '@tauri-apps/api/event';
import { ComponentType, useEffect, useState } from 'react';
import { VscDebugStart, VscDebugPause, VscDebugStop } from 'react-icons/vsc';

const initialTimerState: TimerState = {
  status: TimerStatus.Idle,
  minutes: 0,
  seconds: 0,
};

interface TimerProps {
  compact?: boolean;
}

const Timer: ComponentType<TimerProps> = ({ compact }) => {
  let actionBtnClasses = `w-full flex justify-center items-center rounded-md border ${
    compact ? 'text-lg' : 'text-5xl p-2'
  }  hover:bg-white hover:text-neutral-500`;

  const [timerState, setTimerState] = useState<TimerState>(initialTimerState);

  const listenTimerStateChanged = async () => {
    const unlisten = await listen<TimerState>(
      'timer_state_changed',
      (event) => {
        setTimerState(event.payload);
      }
    );
    return unlisten;
  };

  useEffect(() => {
    const getTimerState = async () => {
      try {
        const tState = await timerGetState();
        setTimerState(tState);
      } catch (error) {
        console.error('Cannot get timer state', error);
      }
    };

    getTimerState();
    listenTimerStateChanged();
  }, []);

  const onStartClick = async () => {
    // Optimistic update
    setTimerState({ ...timerState, status: TimerStatus.Running });
    await timerStart();
  };
  const onPauseClick = async () => {
    // Optimistic update
    setTimerState({ ...timerState, status: TimerStatus.Paused });
    await timerPause();
  };
  const onResetClick = async () => {
    const resetState = await timerReset();
    setTimerState(resetState);
  };

  const renderActionButtons = () => {
    const timerStatus = timerState?.status;
    if (timerStatus === TimerStatus.Idle) {
      return (
        <button onClick={onStartClick} className={actionBtnClasses}>
          <VscDebugStart />
        </button>
      );
    }
    if (timerStatus === TimerStatus.Running) {
      return (
        <>
          <button onClick={onPauseClick} className={actionBtnClasses}>
            <VscDebugPause />
          </button>
          <button onClick={onResetClick} className={actionBtnClasses}>
            <VscDebugStop />
          </button>
        </>
      );
    }
    if (timerStatus === TimerStatus.Paused) {
      return (
        <>
          <button onClick={onStartClick} className={actionBtnClasses}>
            <VscDebugStart />
          </button>
          <button onClick={onResetClick} className={actionBtnClasses}>
            <VscDebugStop />
          </button>
        </>
      );
    }
    if (timerStatus === TimerStatus.Finished) {
      return (
        <button onClick={onResetClick} className={actionBtnClasses}>
          <VscDebugStop />
        </button>
      );
    }
  };

  return (
    <div className='flex grow flex-col justify-center items-center text-center'>
      {timerState ? (
        <>
          <div className={`tabular-nums ${compact ? 'text-2xl' : 'text-9xl'}`}>
            <span>{padWithZero(timerState.minutes)}</span>
            {''} : {''}
            <span>{padWithZero(timerState.seconds)}</span>
          </div>

          {/* Divider */}
          {!compact && <div className='my-10'></div>}

          <div
            className={`w-full flex justify-center items-center gap-1 ${
              compact && 'w-3/4'
            }`}
          >
            {renderActionButtons()}
          </div>
        </>
      ) : (
        <div>Loading...</div>
      )}
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
