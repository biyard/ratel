'use client';

import { Assets } from 'pixi.js';

export function PlayerNameOverlay({
  scale,
  names = ['Rank 1', 'Rank 2', 'Rank 3'],
}: {
  scale: number;
  names?: string[];
}) {
  const shortedName = names.map((name) => {
    if (name.length > 6) {
      return name.slice(0, 6);
    } else if (name.length < 6) {
      return name.padEnd(6, ' ');
    }
    return name;
  });
  return (
    <pixiContainer>
      <pixiContainer>
        <pixiSprite texture={Assets.get('rank-banner')} scale={scale} />
        <pixiText
          text={shortedName[0]}
          x={110 * scale}
          y={118 * scale}
          style={{ fontSize: 32 * scale, wordWrap: true, wordWrapWidth: 150 }}
        />
      </pixiContainer>
      <pixiText
        text={shortedName[1]}
        x={(360 - shortedName[1].length * 18) * scale}
        y={425 * scale}
        style={{
          fontSize: 24 * scale,
        }}
      />
      <pixiText
        text={shortedName[2]}
        x={(360 - shortedName[2].length * 18) * scale}
        y={518 * scale}
        style={{
          fontSize: 24 * scale,
        }}
      />
    </pixiContainer>
  );
}

export function Banner({ scale }: { scale: number }) {
  return <pixiSprite texture={Assets.get('banner')} scale={scale} />;
}

export function VoteBanner({ scale }: { scale: number }) {
  return <pixiSprite texture={Assets.get('vote-header')} scale={scale} />;
}
