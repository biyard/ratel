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
    <div className="flex justify-between items-center bg-neutral-900 text-white px-6 py-3 text-sm font-semibold border-b border-neutral-800">
      <Logo width={32} height={32} />
      <span>{name}</span>
      <Clear
        className="cursor-pointer w-[24px] h-[24px] [&>path]:stroke-neutral-500"
        onClick={onclose}
        fill="white"
      />
    </div>
  );
}
