// 'use client';

import { useTick } from '@pixi/react';
import { Assets, Sprite } from 'pixi.js';
import { useEffect, useRef } from 'react';
import Character from './character';
import { SCALE } from './base';

export default function VotePlayer({
  players = [],
  selectedIndex,
  onSelect,
}: {
  players: { id: number; name: string; description: string }[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
}) {
  return (
    <>
      <pixiContainer>
        {selectedIndex !== null && (
          <pixiText
            position={{ x: 180 * SCALE, y: 50 * SCALE }}
            anchor={{ x: 0.5, y: 0.5 }}
            style={{
              fontSize: 30 * SCALE,
              align: 'center',
              fontWeight: '900',
            }}
            text={players[selectedIndex]?.name}
          />
        )}
        <pixiContainer x={75 * SCALE} y={220 * SCALE}>
          {players.slice(0, 3).map((player, index) => (
            <Vote
              index={index}
              key={player.id}
              x={index * 105 * SCALE}
              selected={selectedIndex === null ? null : selectedIndex === index}
              onClick={() => onSelect(index)}
            />
          ))}
        </pixiContainer>

        <pixiContainer y={370 * SCALE}>
          {selectedIndex === null && (
            <pixiText
              position={{ x: 180 * SCALE, y: 0 }}
              anchor={{ x: 0.5, y: 0.5 }}
              style={{
                fill: '#ffffff',
                fontSize: 20 * SCALE,
                align: 'center',
                fontWeight: 'bold',
              }}
              text="SELECT YOUR PLAYER"
            />
          )}
          {selectedIndex !== null && (
            <pixiText
              position={{ x: 180 * SCALE, y: 0 }}
              anchor={{ x: 0.5, y: 0.5 }}
              style={{
                fill: '#ffffff',
                fontSize: 20 * SCALE,
                align: 'center',
                fontWeight: '900',
                wordWrap: true,
                wordWrapWidth: 300 * SCALE,
              }}
              text={players[selectedIndex].description}
            />
          )}
        </pixiContainer>
      </pixiContainer>
    </>
  );
}

function Vote({
  index,
  selected,
  onClick,
  x,
  y = 0,
}: {
  index: number;
  selected: boolean | null;
  onClick: () => void;
  x: number;
  y?: number;
}) {
  const ref = useRef<Sprite>(null);

  useEffect(() => {}, [selected]);

  useTick((ticker) => {
    const sprite = ref.current;
    if (!sprite) return;

    const targetY = selected ? y - 20 * SCALE : y;

    sprite.position.y += (targetY - sprite.position.y) * 0.1 * ticker.deltaTime;
  });

  let texture = Assets.get('vote-default');
  if (selected === true) {
    texture = Assets.get('vote-selected');
  } else if (selected === false) {
    texture = Assets.get('vote-unselected');
  }

  return (
    <pixiContainer
      ref={ref}
      position={{ x, y }}
      eventMode="static"
      cursor="pointer"
      onClick={onClick}
      onMouseDown={onClick}
      onPointerTap={onClick}
    >
      <pixiSprite texture={texture} anchor={{ x: 0.5, y: 0.5 }} scale={SCALE} />
      <Character index={index} speed={0} x={-50} y={50} />
    </pixiContainer>
  );
}
