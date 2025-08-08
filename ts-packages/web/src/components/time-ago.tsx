import { useTheme } from '@/app/_providers/ThemeProvider';
import { getTimeAgo } from '@/lib/time-utils';
import React from 'react';

export default function TimeAgo({ timestamp }: { timestamp: number }) {
  const { theme } = useTheme();
  return (
    <p
      className={`text-sm align-middle font-light ${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
    >
      {getTimeAgo(timestamp)}
    </p>
  );
}
