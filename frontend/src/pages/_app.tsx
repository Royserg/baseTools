import type { AppProps } from 'next/app';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  const wrapperClasses =
    'rounded-xl text-slate-100 w-screen min-h-screen h-screen border flex flex-col overflow-hidden';

  return (
    <div className={wrapperClasses}>
      <Component {...pageProps} />
    </div>
  );
}

export default MyApp;
