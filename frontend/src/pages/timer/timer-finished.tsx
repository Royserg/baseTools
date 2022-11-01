import type { NextPage } from 'next';
import {
  timerFinishedCloseWindow,
  timerFinishedStartNew,
} from '../../services';

const TimerFinished: NextPage = () => {
  let actionBtnClasses =
    'w-full flex justify-center items-center rounded-md border text-2xl p-2 hover:bg-white hover:text-neutral-500';

  const onStartNewTimerClick = async () => {
    await timerFinishedStartNew();
  };
  const onCloseClick = async () => {
    await timerFinishedCloseWindow();
  };

  return (
    <div className='bg-gray-900 flex flex-col grow justify-center items-center p-2'>
      <h2 className='text-4xl'>Timer Finished</h2>
      <h3 className='text-2xl'>Take a break</h3>

      {/* Divider */}
      <div className='py-5'></div>

      <div className='w-1/2 mx-auto'>
        <button className={actionBtnClasses} onClick={onStartNewTimerClick}>
          Start new timer
        </button>

        <div className='py-2'></div>

        <button className={actionBtnClasses} onClick={onCloseClick}>
          Close
        </button>
      </div>
    </div>
  );
};

export default TimerFinished;
