'use client';

import { Clear, Logo } from '@/components/icons';
import React from 'react';

export function Header({
  name,
  onclose,
}: {
  name: string;
  onclose: () => void;
}) {
  return (
    <div className="flex flex-row items-center w-full bg-neutral-900 light:bg-black text-white px-6 py-3 text-sm font-semibold border-b border-neutral-800">
      <div className="flex flex-row flex-1 justify-start">
        <Logo width={32} height={32} />
      </div>
      <div className="flex flex-row flex-1 justify-center">
        <span>{name}</span>
      </div>
      <div className="flex flex-row flex-1 justify-end">
        <Clear
          className="cursor-pointer w-[24px] h-[24px] [&>path]:stroke-neutral-500"
          onClick={onclose}
          fill="white"
        />
      </div>
    </div>
  );
}
