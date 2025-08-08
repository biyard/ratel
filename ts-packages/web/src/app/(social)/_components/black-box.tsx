import React from 'react';

export interface BlackboxProps {
  children: React.ReactNode;
}

export default function BlackBox({ children }: BlackboxProps) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start  rounded-[10px] px-4 py-5 bg-component-bg border border-transparent light:bg-neutral-50 light:border-neutral-200 `}
    >
      {children}
    </div>
  );
}
