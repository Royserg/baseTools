import { AppBar } from '@components/app-bar';
import { PageWrapper } from '@components/page-wrapper';
import Image from 'next/image';
import Link from 'next/link';
import { BsHourglassSplit } from 'react-icons/bs';

export default function MainWindow() {
  return (
    <PageWrapper>
      {/* Close button */}
      <AppBar />

      {/* Hero section */}
      <div className='w-full'>
        <div className='container mx-auto'>
          <div className='flex flex-col items-center justify-center h-44'>
            <h1 className='text-5xl font-bold text-gray-200'>baseTools</h1>

            <div className='w-1/3 p-4 flex items-center justify-center'>
              <Image
                src={'/assets/base-icon-no-padding.svg'}
                alt='base logo'
                width={170}
                height={100}
              />
            </div>
          </div>
        </div>
      </div>

      {/*  Content: Micro Apps */}
      <div className='rounded-b-xl bg-zinc-800 p-4 grow'>
        {/* Container */}
        <div className='container mt-10 w-1/2 mx-auto flex justify-start'>
          {/* Micro App */}
          <div className='flex justify-center items-center w-24 border p-4 rounded-lg hover:cursor-pointer text-gray-200'>
            <Link href='/timer' className='text-8xl'>
              <div className='text-6xl'>
                <BsHourglassSplit />
              </div>
            </Link>
          </div>
        </div>
      </div>
    </PageWrapper>
  );
}
