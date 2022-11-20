import { emit } from '@tauri-apps/api/event';
import { ChangeEvent, ComponentType, useState } from 'react';
import { closeSunGlassWindow, openSunGlassWindow } from '../services';

interface SunGlassProps {}

export const SunGlass: ComponentType<SunGlassProps> = () => {
  const [isActive, setIsActive] = useState<boolean>(false);
  const [value, setValue] = useState<number>(30);

  // TODO: read state from localStorage
  // useEffect()

  const onSwitchClick = async () => {
    setIsActive((isActive) => !isActive);
    if (isActive) {
      await closeSunGlassWindow();
    }
    await openSunGlassWindow();
  };

  const handleBrightnessChange = async (e: ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    setValue(value);

    await emit('sunglass_brightness_changed', { value });
  };

  return (
    <div className='flex grow justify-center items-center text-center'>
      <button className='border w-1/5 rounded-xl -mb-2' onClick={onSwitchClick}>
        {isActive ? 'off' : 'on'}
      </button>

      <div>
        <div>{value}%</div>
        <input
          className='w-4/5 rounded-lg overflow-hidden appearance-none bg-gray-500'
          type='range'
          value={value}
          min={0}
          max={100}
          onChange={handleBrightnessChange}
        />
      </div>
    </div>
  );
};
