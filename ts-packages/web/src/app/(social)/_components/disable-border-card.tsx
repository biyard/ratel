import React from 'react';
export interface DisableBorderCardProps {
  children: React.ReactNode;
}

/**
 * @deprecated
 * use `<Card variant="secondary" />`
 * ---
 * **import Card from '@/components/card'**
 */
export default function DisableBorderCard({
  children,
}: DisableBorderCardProps) {
  return (
    <div
      className={`flex flex-col w-full justify-start items-start bg-card-bg border border-card-border rounded-[10px] px-4 py-5`}
    >
      {children}
    </div>
  );
}
