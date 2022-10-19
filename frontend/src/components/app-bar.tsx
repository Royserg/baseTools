import type { NextComponentType } from 'next';
import { hideMainWindow } from '../services/tauri';

const AppBar: NextComponentType = () => {
  let navControlClasses = 'rounded-full bg-gray-400 w-3 h-3';

  let onCloseClick = async () => {
    await hideMainWindow();
  };

  return (
    <nav
      className='rounded-t-xl w-full bg-neutral-500 px-4 py-3 flex gap-x-2'
      data-tauri-drag-region='default'
    >
      <button
        id='titlebar-close'
        onClick={onCloseClick}
        className={`${navControlClasses} nav_control_class bg-red-400 flex items-center justify-center`}
      >
        <span className={'text-xs text-red-400 hover:text-black'}>{'x'}</span>
      </button>
      <button disabled={true} className={navControlClasses}></button>
      <button disabled={true} className={navControlClasses}></button>
    </nav>
  );
};

export default AppBar;
