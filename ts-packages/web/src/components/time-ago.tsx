import { getTimeAgo } from '@/lib/time-utils';
import React from 'react';

export default function TimeAgo({ timestamp }: { timestamp: number }) {
  return (
    <p className="text-sm align-middle font-light text-white">
      {getTimeAgo(timestamp)}
    </p>
  );
}
