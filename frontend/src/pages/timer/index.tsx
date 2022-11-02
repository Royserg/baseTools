import type { NextPage } from 'next';
import Link from 'next/link';
import { BsFillCaretLeftFill, BsHourglassSplit } from 'react-icons/bs';
import AppBar from '../../components/app-bar';
import Timer from '../../components/timer';

const TimerPage: NextPage = () => {
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
          <button className='rounded-md p-2 -ml-2 -mt-24 bg-slate-100'>
            <Link href='/'>
              <BsFillCaretLeftFill className='text-zinc-800 text-3xl' />
            </Link>
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
