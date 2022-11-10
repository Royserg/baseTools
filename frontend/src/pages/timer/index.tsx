import { AppBar } from '@components/app-bar';
import { PageWrapper } from '@components/page-wrapper';
import { Timer } from '@components/timer';
import Link from 'next/link';
import { BsFillCaretLeftFill, BsHourglassSplit } from 'react-icons/bs';

export default function TimerWindow() {
  return (
    <PageWrapper bgColorClass='bg-zinc-800'>
      <AppBar />

      {/* Divider */}
      <div className='py-5'></div>

      <main className='flex flex-col grow text-gray-200'>
        <div className='flex justify-center items-center relative'>
          {/* Navigate back */}
          <div className='absolute -left-1 h-full'>
            <Link href='/'>
              <button className='rounded-md h-full p-2 bg-slate-100 flex flex-col justify-center items-center'>
                <BsFillCaretLeftFill className='text-zinc-800 text-3xl' />
              </button>
            </Link>
          </div>

          <h1 className='text-8xl flex justify-center'>
            <BsHourglassSplit />
          </h1>
        </div>

        <div className='w-1/2 h-full mx-auto flex justify-center items-center'>
          <Timer />
        </div>
      </main>
    </PageWrapper>
  );
}
