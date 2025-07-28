// 'use client';

import { useTick } from '@pixi/react';
import { Assets, Sprite } from 'pixi.js';
import { useEffect, useRef } from 'react';
import Character from './character';
import { SCALE } from './base';
import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';

export default function VotePlayer({
  players = [],
  selectedPlayerId,
  onSelect,
}: {
  players: SprintLeaguePlayer[];
  selectedPlayerId: number | null;
  onSelect: (index: number) => void;
}) {
  return (
    <>
      <pixiContainer>
        {selectedPlayerId !== null && (
          <pixiText
            position={{ x: 180 * SCALE, y: 50 * SCALE }}
            anchor={{ x: 0.5, y: 0.5 }}
            style={{
              fontSize: 30 * SCALE,
              align: 'center',
              fontWeight: '900',
            }}
            text={players.find((p) => p.id === selectedPlayerId)?.name || ''}
          />
        )}
        <pixiContainer x={75 * SCALE} y={220 * SCALE}>
          {players.slice(0, 3).map((player, index) => (
            <Vote
              playerId={player.id}
              key={player.id}
              x={index * 105 * SCALE}
              selected={
                selectedPlayerId === null
                  ? null
                  : selectedPlayerId === player.id
              }
              onClick={() => onSelect(player.id)}
            />
          ))}
        </pixiContainer>

        <pixiContainer y={370 * SCALE}>
          {selectedPlayerId === null && (
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
          {selectedPlayerId !== null && (
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
              text={
                players.find((p) => p.id === selectedPlayerId)?.description ||
                ''
              }
            />
          )}
        </pixiContainer>
      </pixiContainer>
    </>
  );
}

function Vote({
  playerId,
  selected,
  onClick,
  x,
  y = 0,
}: {
  playerId: number;
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

  const getTexture = () => {
    let textureName = 'vote-default';
    if (selected === true) {
      textureName = 'vote-selected';
    } else if (selected === false) {
      textureName = 'vote-unselected';
    }

    const texture = Assets.get(textureName);
    if (!texture) {
      console.warn(`Texture ${textureName} not found`);
      return Assets.get('vote-default') || null;
    }
    return texture;
  };

  const texture = getTexture();
  if (!texture) {
    return null;
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
      <Character playerId={playerId} speed={0} x={-50} y={50} />
    </pixiContainer>
  );
}
