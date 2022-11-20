import { useSunGlassStore } from '@stores/sunglass';
import type { NextComponentType } from 'next';
import { closeMainWindow } from '../services/tauri';

export const AppBar: NextComponentType = () => {
  let navControlClasses = 'rounded-full bg-gray-400 w-3 h-3';

  const { setActive } = useSunGlassStore();

  let onCloseClick = async () => {
    setActive(false);
    await closeMainWindow();
  };

  return (
    <nav
      className='rounded-t-xl w-full bg-stone-900 px-4 py-3 flex gap-x-2'
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
