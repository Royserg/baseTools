'use client';

import { PageWrapper } from '@components/page-wrapper';
import { Separator } from '@components/separator';
import { Timer } from '@components/timer';
import { quitApp, showMainWindow } from '@services/tauri/commands';
import { BsHourglassSplit } from 'react-icons/bs';

export default function AppTrayWindow() {
  const trayBtnClasses =
    'w-full hover:bg-gray-700 rounded-lg px-3 my-1 text-left';

  const onAppOpenClick = async () => {
    await showMainWindow();
  };

  const onQuitClick = async () => {
    await quitApp();
  };

  return (
    <PageWrapper>
      <div className='h-full container mx-auto text-gray-200 text-sm p-1 flex flex-col'>
        {/* Open app button */}
        <button className={trayBtnClasses} onClick={onAppOpenClick}>
          Open baseTools
        </button>

        <Separator />

        {/* Timer box */}
        <div className='w-full p-2 grow'>
          <h4 className='text-xs text-gray-400 pl-1'>Timer</h4>
          <div className='flex justify-between items-end gap-1'>
            <span className='text-5xl -mb-1'>
              <BsHourglassSplit />
            </span>
            <Timer compact />
          </div>
        </div>

        <Separator />

        {/* App quit button */}
        <div className='flex justify-between'>
          <button className={trayBtnClasses} onClick={onQuitClick}>
            Quit
          </button>
        </div>
      </div>
    </PageWrapper>
  );
}
