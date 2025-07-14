import { logger } from '@/lib/logger';
import { AnimatedSprite, type Texture, Spritesheet, Assets } from 'pixi.js';
import { useEffect, useRef, useState } from 'react';
import { SCALE } from './base';

export default function Character({
  index,
  x = 0,
  y = 0,
  speed = 1,
  scale = 1,
  selected = false,
}: {
  index: number;
  x?: number;
  y?: number;
  speed?: number;
  scale?: number;
  selected?: boolean;
}) {
  const spriteRef = useRef<AnimatedSprite>(null);
  const [textures, setTextures] = useState<Texture[]>([]);
  useEffect(() => {
    const loadAssets = async () => {
      try {
        let jsonName = `user_${index}_run`;
        if (selected) {
          jsonName = `user_${index}_select`;
        }
        console.log('Loading character assets:', jsonName);
        const sheet = (await Assets.get(jsonName)) as Spritesheet;

        setTextures(Object.values(sheet.textures));
      } catch (error) {
        logger.debug('Asset Load Failed:', error);
      }
    };

    loadAssets();
  }, [index, selected]);

  useEffect(() => {
    if (spriteRef.current && textures.length > 0) {
      spriteRef.current.play();
    }
  }, [textures, spriteRef]);

  if (textures.length === 0) {
    return null;
  }

  return (
    <pixiContainer scale={SCALE}>
      <pixiAnimatedSprite
        anchor={{ x: 0, y: 1 }}
        ref={spriteRef}
        width={100 * scale}
        height={100 * scale}
        animationSpeed={speed}
        textures={textures}
        position={{ x, y }}
      />
    </pixiContainer>
  );
}
