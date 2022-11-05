import { PageWrapper } from '@components/page-wrapper';
import { TimerFinishedActionButtons } from '@components/timer-finished-action-btns';

export default function TimerFinishedWindow() {
  return (
    <PageWrapper>
      <div className='flex flex-col grow justify-center items-center p-2'>
        <h2 className='text-4xl'>Timer Finished</h2>
        <h3 className='text-2xl'>Take a break</h3>

        {/* Divider */}
        <div className='py-5'></div>

        <TimerFinishedActionButtons />
      </div>
    </PageWrapper>
  );
}
