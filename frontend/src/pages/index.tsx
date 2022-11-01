import type { NextPage } from 'next';
import Head from 'next/head';
import { BsHourglassSplit } from 'react-icons/bs';
import AppBar from '../components/app-bar';
import Timer from '../components/timer';

const Home: NextPage = () => {
  return (
    <div className='bg-neutral-500 w-full h-full flex flex-col'>
      <Head>
        <title>BaseApp</title>
        <meta name='description' content='Base application' />
        <link rel='icon' href='/favicon.ico' />
      </Head>

      <AppBar />

      <main className='flex flex-col grow'>
        <h1 className='text-8xl flex justify-center'>
          <BsHourglassSplit />
        </h1>

        <div className='w-1/2 h-full mx-auto flex justify-center items-center'>
          <Timer />
        </div>
      </main>
    </div>
  );
};

export default Home;
