import { PageWrapper } from '@components/page-wrapper';
import { TimerFinishedActionButtons } from '@components/timer-finished-action-btns';
import { BsHourglassBottom } from 'react-icons/bs';

export default function TimerFinishedWindow() {
  return (
    <PageWrapper>
      <div className='h-full flex flex-col grow justify-center items-center p-2'>
        <h2 className='text-4xl flex gap-2'>
          <BsHourglassBottom />
          {''}
          00:00
        </h2>

        {/* Divider */}
        <div className='py-2'></div>

        <TimerFinishedActionButtons />
      </div>
    </PageWrapper>
  );
}
