import React from 'react';

interface PageWrapperProps {
  bgColorClass?: string;
}

export const PageWrapper = ({
  children,
  bgColorClass,
}: { children: React.ReactNode } & PageWrapperProps) => {
  return (
    <div
      className={`${
        bgColorClass || 'bg-stone-900'
      } text-gray-200 w-screen h-screen flex flex-col rounded-xl border`}
    >
      {children}
    </div>
  );
};
