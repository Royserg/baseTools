import type { NextPage } from 'next';
import Head from 'next/head';
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
        <h1 className='text-4xl text-center underline'>Time Chamber</h1>

        <Timer />
      </main>
    </div>
  );
};

export default Home;
