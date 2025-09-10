import React from 'react';

export interface BlackboxProps {
  children: React.ReactNode;
  isWhite?: boolean;
}

export default function BlackBox({ children, isWhite }: BlackboxProps) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start ${isWhite ? 'bg-card-bg-secondary border border-card-border-secondary' : 'bg-card-bg'} border border-card-border rounded-[10px] px-4 py-5`}
    >
      {children}
    </div>
  );
}
