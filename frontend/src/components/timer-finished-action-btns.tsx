'use client';

import {
  timerFinishedCloseWindow,
  timerFinishedStartNew,
} from '@services/tauri/commands';

export const TimerFinishedActionButtons = () => {
  let actionBtnClasses =
    'w-full flex justify-center items-center rounded-md border text-2xl p-2 hover:bg-white hover:text-neutral-500';

  const onStartNewTimerClick = async () => {
    await timerFinishedStartNew();
  };
  const onCloseClick = async () => {
    await timerFinishedCloseWindow();
  };

  return (
    <div className='w-1/2 mx-auto'>
      <button className={actionBtnClasses} onClick={onStartNewTimerClick}>
        Start new timer
      </button>

      <div className='py-2'></div>

      <button className={actionBtnClasses} onClick={onCloseClick}>
        Close
      </button>
    </div>
  );
};
