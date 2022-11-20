import { EVENTS_SUNGLASS } from '@shared/events';
import { useSunGlassStore } from '@stores/sunglass';
import { emit } from '@tauri-apps/api/event';
import { ChangeEvent, ComponentType, useEffect, useState } from 'react';
import { closeSunGlassWindow, openSunGlassWindow } from '../services';

interface SunGlassProps {}

export const SunGlass: ComponentType<SunGlassProps> = () => {
  const [loading, setLoading] = useState(true);

  const { active, setActive, opacity, setOpacity } = useSunGlassStore(
    (state) => state
  );

  useEffect(() => {
    if (active !== undefined && opacity !== undefined) {
      setLoading(false);
    }
  }, [active, opacity]);

  const onSwitchClick = async () => {
    if (active) {
      await closeSunGlassWindow();
    }
    await openSunGlassWindow();

    setActive(!active);
  };

  const handleBrightnessChange = async (e: ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    setOpacity(value);
    await emit(EVENTS_SUNGLASS.BRIGHTNESS_CHANGED, { value });
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div className='flex grow justify-center items-center text-center'>
      <button className='border w-1/5 rounded-xl -mb-2' onClick={onSwitchClick}>
        {active ? 'on' : 'off'}
      </button>

      <div className='relative'>
        {/* Disabled slider overlay */}
        {!active && (
          <div className='absolute w-full rounded-xl top-0 bottom-0 bg-gray-900 opacity-50'></div>
        )}

        <div>{opacity}%</div>
        <input
          className='w-4/5 rounded-lg overflow-hidden appearance-none bg-gray-500'
          type='range'
          value={opacity}
          min={0}
          max={100}
          disabled={!active}
          onChange={handleBrightnessChange}
        />
      </div>
    </div>
  );
};
