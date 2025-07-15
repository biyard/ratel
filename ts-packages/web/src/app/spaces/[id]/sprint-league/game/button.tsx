import { Assets } from 'pixi.js';
import { SCALE } from './base';

export function StartButton({
  onClick,
  x = 0,
  y = 0,
}: {
  onClick: () => void;
  x?: number;
  y?: number;
}) {
  return (
    <pixiSprite
      position={{ x: x * SCALE, y: y * SCALE }}
      eventMode="static"
      cursor="pointer"
      texture={Assets.get('button-start')}
      scale={SCALE}
      onPointerTap={onClick}
      onClick={onClick}
    />
  );
}

export function RepostButton({
  onClick,
  x = 0,
  y = 0,
}: {
  onClick: () => void;
  x?: number;
  y?: number;
}) {
  return (
    <pixiSprite
      position={{ x: x * SCALE, y: y * SCALE }}
      eventMode="static"
      anchor={{ x: 1, y: 1 }}
      cursor="pointer"
      texture={Assets.get('button-repost')}
      scale={0.8 * SCALE}
      onClick={onClick}
      onPointerTap={onClick}
    />
  );
}

export function VoteBackButton({
  onVote,
  onBack,
}: {
  onVote: () => void;
  onBack: () => void;
}) {
  return (
    <pixiContainer y={540 * SCALE} scale={SCALE}>
      <pixiSprite
        eventMode="static"
        cursor="pointer"
        onClick={onVote}
        onPointerTap={onVote}
        texture={Assets.get('button-vote')}
        x={165}
        scale={1}
      />
      <pixiSprite
        eventMode="static"
        cursor="pointer"
        onClick={onBack}
        onPointerTap={onBack}
        texture={Assets.get('button-back')}
        scale={1}
      />
    </pixiContainer>
  );
}
