'use client';

import dynamic from 'next/dynamic';

const Base = dynamic(() => import('./stage'), {
  ssr: false,
});

//Set Name as Konva to avoid confusion
export default Base;
