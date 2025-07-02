'use client';

import React from 'react';
import Image from 'next/image';

export default function SprintLeaguePage() {
  const backgroundImagePath = '/images/sabana-bg.png';
  const backgroundLabel = '/images/sabana-sheet.png';

  const leeJae = '/images/lee-jae.gif';
  const leeJun = '/images/lee-jun.gif';
  const kimMoon = '/images/kim-moon.gif';

  return (
    <div className="relative w-full h-[80vh] max-tablet:h-[70vh] overflow-hidden">
      <Image
        src={backgroundImagePath}
        alt="Sabana Background"
        fill
        className="object-full"
        priority
      />

      <div className="absolute top-4 max-tablet:top-[50px] left-1/2 -translate-x-1/2 z-10 max-tablet:w-full">
        <div className="w-[500px] h-fit max-tablet:w-full">
          <Image
            src={backgroundLabel}
            alt="Sprint League Label"
            width={10000}
            height={10000}
          />
        </div>
      </div>

      <div className="absolute left-[1%] bottom-[300px] z-20 max-tablet:w-[150px] max-tablet:h-[150px] max-tablet:bottom-[280px]">
        <Image src={leeJae} alt="Lee Jae" width={300} height={300} />
      </div>

      <div className="absolute left-[1%] bottom-[150px] z-20 max-tablet:w-[150px] max-tablet:h-[150px]">
        <Image src={leeJun} alt="Lee Jun" width={300} height={300} />
      </div>

      <div className="absolute left-[1%] bottom-[10px] z-20 max-tablet:w-[150px] max-tablet:h-[150px]">
        <Image src={kimMoon} alt="Kim Moon" width={300} height={300} />
      </div>
    </div>
  );
}
