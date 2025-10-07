// @ts-nocheck
'use client';

import { Assets } from 'pixi.js';

export function PlayerNameOverlay({
  scale,
  names = ['Rank 1', 'Rank 2', 'Rank 3'],
}: {
  scale: number;
  names?: string[];
}) {
  const normalizedNames = [0, 1, 2].map((i) => {
    const raw = (names[i] ?? '').slice(0, 6);
    return raw.padEnd(6, ' ');
  });

  return (
    <pixiContainer>
      <pixiContainer>
        <pixiSprite texture={Assets.get('rank-banner')} scale={scale} />
        {/* @ts-expect-error - Pixi.js types incompatibility with @pixi/react */}
        <pixiText
          text={normalizedNames[0]}
          x={110 * scale}
          y={118 * scale}
          style={{ fontSize: 32 * scale, wordWrap: true, wordWrapWidth: 150 }}
        />
      </pixiContainer>
      {/* @ts-expect-error - Pixi.js types incompatibility with @pixi/react */}
      <pixiText
        text={normalizedNames[1]}
        x={(360 - normalizedNames[1].length * 18) * scale}
        y={425 * scale}
        style={{
          fontSize: 24 * scale,
        }}
      />
      {/* @ts-expect-error - Pixi.js types incompatibility with @pixi/react */}
      <pixiText
        text={normalizedNames[2]}
        x={(360 - normalizedNames[2].length * 18) * scale}
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
