'use client';

import { useEffect, useState } from 'react';
import { getTimeAgo } from '@/lib/time-utils';

export default function TimeAgo({ timestamp }: { timestamp: number }) {
  const [displayTime, setDisplayTime] = useState<string | null>(null);

  useEffect(() => {
    const updateTime = () => {
      setDisplayTime(getTimeAgo(timestamp));
    };

    updateTime();

    const interval = setInterval(updateTime, 60_000);

    return () => clearInterval(interval);
  }, [timestamp]);

  if (!displayTime) return null;

  return (
    <span className="font-light text-white text-sm/[14px]">{displayTime}</span>
  );
}
