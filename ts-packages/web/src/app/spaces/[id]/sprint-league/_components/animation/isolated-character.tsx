import { Application, extend, useApplication } from '@pixi/react';
import { Texture, AnimatedSprite } from 'pixi.js';
import { useEffect, useRef, useState } from 'react';
import { pixiAssetManager } from './assets';
import { logger } from '@/lib/logger';

extend({ AnimatedSprite });

export default function IsolatedCharacter({
  alias,
}: {
  alias: string;
  size?: number;
  scale?: number;
  animationSpeed?: number;
  isPlaying?: boolean;
}) {
  const ref = useRef<HTMLDivElement>(null);

  const [textures, setTextures] = useState<Texture[]>([]);
  useEffect(() => {
    const loadAssets = async () => {
      try {
        const aliasRun = `${alias}_run`;
        const sheet = await pixiAssetManager.getAsset(aliasRun);
        if (!sheet) {
          throw new Error(`Spritesheet not found: ${aliasRun}`);
        }
        setTextures(Object.values(sheet.textures));
      } catch (error) {
        logger.debug('Asset Load Failed:', error);
      }
    };

    loadAssets();
  }, [alias]);

  if (!alias || textures.length === 0) {
    return null;
  }

  return (
    <div className="w-full h-full" ref={ref}>
      <Application resizeTo={ref} backgroundAlpha={0}>
        <Sprite textures={textures} />
      </Application>
    </div>
  );
}

function Sprite({ textures }: { textures: Texture[] }) {
  const { app } = useApplication();
  const spriteRef = useRef<AnimatedSprite>(null);
  useEffect(() => {
    if (spriteRef.current && textures.length > 0) {
      spriteRef.current.play();
    }
  }, [textures, spriteRef]);
  if (!app) {
    return null;
  }

  return (
    <pixiAnimatedSprite
      textures={textures}
      anchor={{ x: 0.5, y: 0.5 }}
      scale={app.screen.width / 360}
      position={{
        x: app.screen.width / 2,
        y: app.screen.height / 2,
      }}
      ref={spriteRef}
      animationSpeed={0.3}
    />
  );
}
