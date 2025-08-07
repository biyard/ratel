import React from 'react';

export default function BlackBox({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-col w-full justify-start items-start px-4 py-5 bg-[#191919] rounded-lg gap-5">
      <div className="font-bold text-white text-[15px]/[20px]">{title}</div>
      {children}
    </div>
  );
}
