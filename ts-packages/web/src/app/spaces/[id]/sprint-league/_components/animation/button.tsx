import { Assets } from 'pixi.js';
import { useCallback, useEffect, useRef, useState } from 'react';

export function StartButton({
  onClick,
  x = 0,
  y = 0,
  scale,
}: {
  onClick: () => void;
  x?: number;
  y?: number;
  scale: number;
}) {
  return (
    <pixiSprite
      position={{ x: x * scale, y: y * scale }}
      eventMode="static"
      cursor="pointer"
      texture={Assets.get('button-start')}
      scale={scale}
      onPointerTap={onClick}
    />
  );
}

export function RepostButton({
  onClick,
  x = 0,
  y = 0,
  scale,
}: {
  onClick: () => void;
  x?: number;
  y?: number;
  scale: number;
}) {
  return (
    <pixiSprite
      position={{ x: x * scale, y: y * scale }}
      eventMode="static"
      anchor={{ x: 1, y: 1 }}
      cursor="pointer"
      texture={Assets.get('button-repost')}
      scale={0.8 * scale}
      onPointerTap={onClick}
    />
  );
}

export function VoteBackButton({
  onVote,
  onBack,
  scale,
}: {
  onVote: () => void;
  onBack: () => void;
  scale: number;
}) {
  const [isVoteClicked, setIsVoteClicked] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const handleVote = useCallback(() => {
    if (isVoteClicked) return;

    setIsVoteClicked(true);
    onVote();
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    timeoutRef.current = setTimeout(() => {
      setIsVoteClicked(false);
      timeoutRef.current = null;
    }, 1000);
  }, [isVoteClicked, onVote]);

  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return (
    <pixiContainer y={540 * scale} scale={scale}>
      <pixiSprite
        eventMode="static"
        cursor="pointer"
        onPointerTap={handleVote}
        texture={Assets.get('button-vote')}
        x={165}
        scale={1}
      />
      <pixiSprite
        eventMode="static"
        cursor="pointer"
        onPointerTap={onBack}
        texture={Assets.get('button-back')}
        scale={1}
      />
    </pixiContainer>
  );
}
