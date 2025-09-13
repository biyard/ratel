'use client';

import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';
import dynamic from 'next/dynamic';
import { useEffect, useRef, useState } from 'react';

const Base = dynamic(() => import('./stage'), {
  ssr: false,
});

//Set Name as Konva to avoid confusion

export const VIEWPORT_WIDTH = 360;
export const VIEWPORT_HEIGHT = 640;
export const CHARACTER_SIZE = 200;

export default function Game({
  initialStatus = Status.BEFORE_START,
  players,
  onVote,
  disabled,
}: {
  initialStatus: Status;
  players: SprintLeaguePlayer[];
  onVote: (playerId: number) => Promise<void>;
  disabled: boolean;
}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [containerSize, setContainerSize] = useState({
    width: VIEWPORT_WIDTH,
    height: VIEWPORT_HEIGHT,
  });

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    const ro = new ResizeObserver((entries) => {
      const r = entries[0].contentRect;
      setContainerSize({ width: r.width, height: r.height });
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  // Always width / height to maintain aspect ratio. but not exceed container size

  const scale = Math.min(
    containerSize.width / VIEWPORT_WIDTH,
    containerSize.height / VIEWPORT_HEIGHT,
  );
  const adjustedWidth = VIEWPORT_WIDTH * scale;
  const adjustedHeight = VIEWPORT_HEIGHT * scale;
  return (
    <div
      ref={containerRef}
      className="w-full h-full flex justify-center items-center"
    >
      <Base
        width={adjustedWidth}
        height={adjustedHeight}
        onVote={onVote}
        disabled={disabled}
        initialStatus={initialStatus}
        players={players}
      />
    </div>
  );
}

export enum Status {
  BEFORE_START = 0,
  VOTE = 1,
  AFTER_VOTE = 2,
  GAME_END = 3,
}
