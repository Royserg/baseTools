import { useEffect, useState } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

export default function SunGlassOverlay() {
  const [opacity, setOpacity] = useState<number>(30);

  useEffect(() => {
    let unlisten: UnlistenFn;

    listen<string>('sunglass_brightness_changed', (e) => {
      const payload: { value: number } = JSON.parse(e.payload);
      setOpacity(payload.value);
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
    />
  );
}
