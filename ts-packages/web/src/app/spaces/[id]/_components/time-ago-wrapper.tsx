'use client';

import dynamic from 'next/dynamic';

const TimeAgo = dynamic(() => import('./time-ago'), { ssr: false });
export default TimeAgo;
