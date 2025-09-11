import React from 'react';

export interface SpaceBlackboxProps {
  children: React.ReactNode;
}

export default function SpaceBlackboxProps({ children }: SpaceBlackboxProps) {
  return (
    <div className="flex flex-col w-full justify-start items-start bg-space-box-bg border border-space-box-border rounded-[10px] px-4 py-5">
      {children}
    </div>
  );
}
