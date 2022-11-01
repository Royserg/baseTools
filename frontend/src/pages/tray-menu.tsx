import type { NextPage } from 'next';
import { BsHourglassSplit } from 'react-icons/bs';
import Timer from '../components/timer';
import { quitApp, showMainWindow } from '../services/tauri';

const TrayMenu: NextPage = () => {
  const onAppOpenClick = async () => {
    await showMainWindow();
  };

  const onQuitClick = async () => {
    await quitApp();
  };

  return (
    <div className='bg-gray-900 flex flex-col grow justify-between items-center p-2'>
      <div className='w-full'>
        <div className='flex justify-between items-end gap-1'>
          <span className='text-5xl -mb-1'>
            <BsHourglassSplit />
          </span>
            <Timer compact />
        </div>
      </div>

      {/* Divider */}
      <div className='w-full p-3'></div>

      {/* Menu buttons */}
      <div className='rounded-md flex flex-col gap-1 w-full text-sm'>
        {/* Divider */}
        <div className='border'></div>

        <div className='flex justify-between'>
          <button
            className='w-full hover:bg-gray-700 rounded-sm'
            onClick={onAppOpenClick}
          >
            Open App
          </button>

          {/* Divider */}
          <div className='border'></div>

          <button
            className='w-full hover:bg-gray-700 rounded-sm'
            onClick={onQuitClick}
          >
            Quit
          </button>
        </div>
      </div>
    </div>
  );
};

export default TrayMenu;
