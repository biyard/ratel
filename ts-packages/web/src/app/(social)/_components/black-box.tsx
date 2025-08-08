import { useTheme } from '@/app/_providers/ThemeProvider';
import React from 'react';

export interface BlackboxProps {
  children: React.ReactNode;
}

export default function BlackBox({ children }: BlackboxProps) {
  const { theme } = useTheme();
  return (
    <div
      className={`flex flex-col w-full justify-start items-start  rounded-[10px] px-4 py-5 ${theme === 'light' ? 'bg-neutral-50 border border-neutral-200' : 'bg-component-bg'}`}
    >
      {children}
    </div>
  );
}
