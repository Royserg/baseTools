import {
  timerFinishedCloseWindow,
  timerFinishedStartNew,
} from '@services/tauri/commands';

export const TimerFinishedActionButtons = () => {
  let actionBtnClasses =
    'w-full flex justify-center items-center rounded-md border text-md p-1 hover:bg-white hover:text-neutral-500 mb-2';

  const onStartNewTimerClick = async () => {
    await timerFinishedStartNew();
  };
  const onCloseClick = async () => {
    await timerFinishedCloseWindow();
  };

  return (
    <div className='w-full px-2'>
      {/* <button className={actionBtnClasses} onClick={onBreakClick}>
        Break
      </button> */}

      <button className={actionBtnClasses} onClick={onStartNewTimerClick}>
        New Timer
      </button>

      <button className={actionBtnClasses} onClick={onCloseClick}>
        Close
      </button>
    </div>
  );
};
