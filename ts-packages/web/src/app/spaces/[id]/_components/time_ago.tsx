'use client';

import { useEffect, useState } from 'react';
import { getTimeAgo } from '@/lib/time-utils';

export default function TimeAgo({ timestamp }: { timestamp: number }) {
  const [displayTime, setDisplayTime] = useState(getTimeAgo(timestamp));

  useEffect(() => {
    const interval = setInterval(() => {
      setDisplayTime(getTimeAgo(timestamp));
    }, 1000);

    return () => clearInterval(interval);
  }, [timestamp]);

  return (
    <span className="font-light text-white text-sm/[14px]">{displayTime}</span>
  );
}
