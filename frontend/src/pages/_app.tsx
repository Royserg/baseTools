import '../styles/globals.css';
import type { AppProps } from 'next/app';

function MyApp({ Component, pageProps }: AppProps) {
  const wrapperClasses =
    'rounded-xl text-white w-screen min-h-screen h-screen border flex flex-col overflow-hidden';

  return (
    <div className={wrapperClasses}>
      <Component {...pageProps} />
    </div>
  );
}

export default MyApp;
