import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import { BsFillCaretLeftFill, BsHourglassSplit } from 'react-icons/bs';
import AppBar from '../../components/app-bar';
import Timer from '../../components/timer';

const TimerPage: NextPage = () => {
  const router = useRouter();

  return (
    <div className='bg-zinc-800 w-full h-full flex flex-col'>
      <AppBar />

      {/* Divider */}
      <div className='py-5'></div>

      <main className='flex flex-col grow'>
        <h1 className='text-8xl flex justify-center'>
          <BsHourglassSplit />
        </h1>

        {/* Navigate back */}
        <div className='flex'>
          <button
            className='rounded-md p-2 -ml-2 -mt-24 bg-slate-100'
            onClick={() => router.back()}
          >
            <BsFillCaretLeftFill className='text-zinc-800 text-3xl' />
          </button>
        </div>

        <div className='w-1/2 h-full mx-auto flex justify-center items-center'>
          <Timer />
        </div>
      </main>
    </div>
  );
};

export default TimerPage;
