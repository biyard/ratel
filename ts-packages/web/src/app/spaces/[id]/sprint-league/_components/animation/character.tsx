'use client';

import { logger } from '@/lib/logger';
import { AnimatedSprite, Assets, type Texture } from 'pixi.js';
import { useEffect, useRef, useState } from 'react';
import { pixiAssetManager } from './assets';

const RANK_POSITION = [
  { x: 15, y: 470, scale: 3 },
  { x: 0, y: 550, scale: 1.5 },
  { x: 0, y: 640, scale: 1.5 },
];

export default function Character({
  alias,
  x = 0,
  y = 0,
  speed = 1,
  scale,
  characterScale = 1,
  selected = false,
  index = 0,
  isFinished = false,
}: {
  alias: string;
  x?: number;
  y?: number;
  speed?: number;
  scale: number;
  characterScale?: number;
  selected?: boolean;
  index?: number;
  isFinished?: boolean;
}) {
  const spriteRef = useRef<AnimatedSprite>(null);
  const [textures, setTextures] = useState<Texture[]>([]);
  useEffect(() => {
    const loadAssets = async () => {
      if (isFinished) {
        return;
      }
      try {
        let jsonName = `${alias}_run`;
        if (selected) {
          jsonName = `${alias}_selected`;
        }
        const sheet = await pixiAssetManager.getAsset(jsonName);
        if (!sheet) {
          throw new Error(`Spritesheet not found: ${jsonName}`);
        }
        setTextures(Object.values(sheet.textures));
      } catch (error) {
        logger.debug('Asset Load Failed:', error);
      }
    };

    loadAssets();
  }, [alias, isFinished, selected]);

  useEffect(() => {
    if (spriteRef.current && textures.length > 0) {
      spriteRef.current.play();
    }
  }, [textures, spriteRef]);

  if (isFinished) {
    const v = index === 0 ? `${alias}_win` : `${alias}_lose`;

    return (
      <pixiContainer scale={scale}>
        <pixiSprite
          scale={scale}
          texture={Assets.get(v)}
          anchor={{ x: 0, y: 1 }}
          ref={spriteRef}
          width={100 * RANK_POSITION[index].scale}
          height={100 * RANK_POSITION[index].scale}
          position={{ x: RANK_POSITION[index].x, y: RANK_POSITION[index].y }}
        />
      </pixiContainer>
    );
  }
  if (textures.length === 0) {
    return null;
  }
  return (
    <pixiContainer scale={scale}>
      <pixiAnimatedSprite
        // label={alias}
        anchor={{ x: 0, y: 1 }}
        ref={spriteRef}
        width={100 * characterScale}
        height={100 * characterScale}
        animationSpeed={speed}
        textures={textures}
        position={{ x, y }}
      />
    </pixiContainer>
  );
}
