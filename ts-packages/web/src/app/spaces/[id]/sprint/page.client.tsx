'use client';

import Image from 'next/image';
import SprintLeagueCanvas from './components/league-canvas';
import { useEffect, useState } from 'react';

export default function SprintLeaguePage() {
  const [canvasHeight, setCanvasHeight] = useState(0);

  useEffect(() => {
    const handleResize = () => {
      const tablet = window.innerWidth <= 768;
      setCanvasHeight(window.innerHeight * (tablet ? 0.7 : 0.8));
    };

    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return (
    <div className="relative w-full h-[80vh] max-tablet:h-[70vh] overflow-hidden">
      <Image
        src="/images/sabana-bg.png"
        alt="Sabana Background"
        fill
        className="object-fit z-0"
        priority
      />

      <div className="absolute top-4 left-1/2 -translate-x-1/2 z-10">
        <div className="w-[500px] h-fit max-tablet:w-full">
          <Image
            src="/images/sabana-sheet.png"
            alt="Sprint League Label"
            width={10000}
            height={10000}
          />
        </div>
      </div>

      {canvasHeight > 0 && (
        <SprintLeagueCanvas
          height={canvasHeight}
          width={window.innerWidth}
          targetPercents={[0.5, 0.6, 1]}
        />
      )}
    </div>
  );
}
