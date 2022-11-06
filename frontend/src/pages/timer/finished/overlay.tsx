import { useEffect, useState } from 'react';

// Tailwind opacity classes
// https://tailwindcss.com/docs/opacity
const opacityValues = [
  'opacity-30',
  'opacity-40',
  'opacity-50',
  'opacity-60',
  'opacity-70',
  'opacity-75',
  'opacity-80',
  'opacity-90',
];
const opacityChangeDelay = 1000 * 20; // 20 seconds

export default function TimerFinishedOverlay() {
  const [opacityIdx, setOpacityIdx] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setOpacityIdx((idx) => idx + 1);
    }, opacityChangeDelay);
    return () => clearInterval(id);
  }, []);

  const opacityClass =
    opacityIdx < opacityValues.length
      ? opacityValues[opacityIdx]
      : opacityValues.at(-1);

  return (
    <div
      className={`${opacityClass} bg-black w-screen h-screen flex justify-center items-end pb-8`}
    >
      <h1 className='text-6xl text-gray-200 text-center font-extrabold'>
        Take a Break
      </h1>
    </div>
  );
}
