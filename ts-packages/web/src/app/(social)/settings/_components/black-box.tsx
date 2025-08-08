import React from 'react';

export default function BlackBox({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start px-4 py-5  rounded-lg gap-5 bg-component-bg border border-transparent light:bg-neutral-50 light:border-neutral-200`}
    >
      <div
        className={`font-bold text-[15px]/[20px] text-white light:text-neutral-800`}
      >
        {title}
      </div>
      {children}
    </div>
  );
}
