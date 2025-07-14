import { Assets } from 'pixi.js';
import { SCALE } from './base';

export function PlayerNameOverlay({
  names = ['Rank 1', 'Rank 2', 'Rank 3'],
}: {
  names?: string[];
}) {
  return (
    <pixiContainer>
      <pixiContainer>
        <pixiSprite texture={Assets.get('rank-banner')} scale={SCALE} />
        <pixiText
          text={names[0]}
          x={110 * SCALE}
          y={118 * SCALE}
          style={{ fontSize: 32 * SCALE }}
        />
      </pixiContainer>
      <pixiText
        text={names[1]}
        x={200 * SCALE}
        y={425 * SCALE}
        style={{ fontSize: 24 * SCALE }}
      />
      <pixiText
        text={names[2]}
        x={160 * SCALE}
        y={518 * SCALE}
        style={{ fontSize: 24 * SCALE }}
      />
    </pixiContainer>
  );
}

export function Banner() {
  return <pixiSprite texture={Assets.get('banner')} scale={SCALE} />;
}

export function VoteBanner() {
  return <pixiSprite texture={Assets.get('vote-header')} scale={SCALE} />;
}
