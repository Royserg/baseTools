import { EVENTS_SUNGLASS } from '@shared/events';
import { useSunGlassStore } from '@stores/sunglass';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export default function SunGlassOverlay() {
  const [opacity, setOpacity] = useState<number>(30);
  const storeOpacity = useSunGlassStore((state) => state.opacity);

  useEffect(() => {
    if (storeOpacity) {
      setOpacity(storeOpacity);
    }
  }, [storeOpacity]);

  useEffect(() => {
    let unlisten: UnlistenFn;

    listen<{ value: number }>(EVENTS_SUNGLASS.BRIGHTNESS_CHANGED, (e) => {
      const value = e.payload.value;
      setOpacity(value);
    }).then((unlistenHandler) => (unlisten = unlistenHandler));

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const wrapperStyles = { opacity: opacity / 100 };

  return (
    <div
      className='bg-black w-screen h-screen flex justify-center items-center pb-8'
      style={wrapperStyles}
    ></div>
  );
}
