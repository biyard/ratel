import { useEffect, useRef, useState } from 'react';
import SprintLeaguePlayer from '../../types/sprint-league-player';
import Base from './stage';
import { VIEWPORT_HEIGHT, VIEWPORT_WIDTH } from './constants';

//Set Name as Konva to avoid confusion

export interface SprintLeagueGameProps {
  initialStatus: Status;
  players?: SprintLeaguePlayer[];
  onVote: (playerSk: string) => Promise<void>;
  disabled: boolean;
}
export default function Game({
  initialStatus = Status.BEFORE_START,
  players,
  onVote,
  disabled,
}: SprintLeagueGameProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [containerWidth, setContainerWidth] = useState(VIEWPORT_WIDTH);
  // const [containerHeight, setContainerHeight] = useState(VIEWPORT_HEIGHT);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    const ro = new ResizeObserver((entries) => {
      const entry = entries[0];

      if (entry.contentBoxSize) {
        const contentBox = entry.contentBoxSize[0];
        setContainerWidth(Math.max(contentBox.inlineSize, VIEWPORT_WIDTH));
      } else {
        const r = entry.contentRect;
        setContainerWidth(Math.max(r.width, VIEWPORT_WIDTH));
      }
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  const adjustedWidth = containerWidth;
  const adjustedHeight = VIEWPORT_HEIGHT * (containerWidth / VIEWPORT_WIDTH);
  return (
    <div className="flex justify-center w-full">
      <div ref={containerRef} className="w-full max-w-120">
        <Base
          width={adjustedWidth}
          height={adjustedHeight}
          onVote={onVote}
          disabled={disabled}
          initialStatus={initialStatus}
          players={players}
        />
      </div>
    </div>
  );
}

export const Status = {
  BEFORE_START: 0,
  VOTE: 1,
  AFTER_VOTE: 2,
  GAME_END: 3,
} as const;

export type Status = (typeof Status)[keyof typeof Status];
