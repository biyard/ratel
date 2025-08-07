import React from 'react';

export default function ContentBox({
  onClick,
  canClicked,
  children,
}: {
  onClick: () => void;
  canClicked: boolean;
  children: React.ReactNode;
}) {
  return (
    <div
      className={`${canClicked ? 'cursor-pointer' : ''} flex flex-row w-full justify-start items-center p-5 rounded-sm border border-neutral-800 bg-transparent`}
      onClick={onClick}
    >
      {children}
    </div>
  );
}
