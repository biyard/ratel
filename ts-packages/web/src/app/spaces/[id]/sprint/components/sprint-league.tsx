'use client';

'use strict';
import React, { useEffect, useRef, useState } from 'react';
import type { Spritesheet, Texture } from 'pixi.js';

import {
  Container,
  AnimatedSprite,
  Graphics,
  Sprite,
  Assets,
  Text,
} from 'pixi.js';
import { extend, Application, useApplication } from '@pixi/react';
import { initDevtools } from '@pixi/devtools';

import { logger } from '@/lib/logger';

extend({
  Container,
  Graphics,
  Sprite,
  AnimatedSprite,
  Text,
});

function DevTool() {
  const { app } = useApplication();
  useEffect(() => {
    initDevtools({ app });
  }, [app]);
  return <></>;
}

const characterBundle = [
  {
    alias: 'lee-jun',
    src: '/images/lee-jun.json',
  },
  {
    alias: 'kim-moon',
    src: '/images/kim-moon.json',
  },
  {
    alias: 'lee-jae',
    src: '/images/lee-jae.json',
  },
];

const backgroundBundle = [
  {
    alias: 'background-image',
    src: '/images/sabana-bg.png',
  },
  {
    alias: 'background-sheet',
    src: '/images/sabana-sheet.png',
  },
  {
    alias: 'background-move',
    src: '/images/sabana-move.png',
  },
];

export function Background({
  names = ['Rank 1', 'Rank 2', 'Rank 3'],
}: {
  names?: string[];
}) {
  const { app } = useApplication();

  return (
    <pixiContainer>
      <pixiSprite
        texture={Assets.get('background-image')}
        width={app.renderer?.width}
        height={app.renderer?.height}
      />
      <pixiContainer>
        <pixiSprite
          texture={Assets.get('background-sheet')}
          width={app.renderer?.width}
          height={app.renderer?.width * 0.5}
        >
          <pixiText text={names[0]} x={110} y={120} style={{ fontSize: 32 }} />
        </pixiSprite>
      </pixiContainer>
      <pixiText text={names[1]} x={200} y={425} style={{ fontSize: 24 }} />
      <pixiText text={names[2]} x={160} y={518} style={{ fontSize: 24 }} />
    </pixiContainer>
  );
}

const rankConfig: Record<
  number,
  { x: number; y: number; speed: number; scale: number }
> = {
  1: {
    x: 150,
    y: 300,
    speed: 0.5,
    scale: 1,
  },
  2: {
    x: 100,
    y: 450,
    speed: 0.3,
    scale: 0.5,
  },
  3: {
    x: 50,
    y: 550,
    speed: 0.1,
    scale: 0.5,
  },
};

export function Character({
  jsonPath,
  rank,
}: {
  jsonPath: string;
  rank: number;
}) {
  const spriteRef = useRef<AnimatedSprite>(null);
  const [textures, setTextures] = useState<Texture[]>([]);
  useEffect(() => {
    const loadAssets = async () => {
      try {
        const sheet = (await Assets.get(jsonPath)) as Spritesheet;
        logger.debug('Asset Loaded:', Object.values(sheet.textures));
        setTextures(Object.values(sheet.textures));
      } catch (error) {
        logger.debug('Asset Load Failed:', error);
      }
    };

    loadAssets();
  }, [jsonPath]);

  useEffect(() => {
    if (spriteRef.current && textures.length > 0) {
      spriteRef.current.play();
    }
  }, [textures, spriteRef]);

  if (textures.length === 0) {
    return null;
  }

  const { x, y, speed, scale } = rankConfig[rank] || {
    x: 0,
    y: 0,
    speed: 1,
    scale: 1,
  };
  return (
    <pixiContainer>
      <pixiAnimatedSprite
        anchor={0.5}
        scale={scale}
        ref={spriteRef}
        animationSpeed={speed}
        textures={textures}
        position={{ x: x, y: y }}
      />
    </pixiContainer>
  );
}

export default function Base({ children }: { children: React.ReactNode }) {
  const [initialized, setInitialized] = React.useState(false);
  useEffect(() => {
    const loadAssets = async () => {
      try {
        await Assets.init({
          manifest: {
            bundles: [
              {
                name: 'characters',
                assets: characterBundle,
              },
              {
                name: 'background',
                assets: backgroundBundle,
              },
            ],
          },
        });

        await Assets.loadBundle(['background', 'characters']);

        setInitialized(true);
      } catch (error) {
        console.error('Asset Load Failed:', error);
      }
    };

    loadAssets();

    return () => {
      Assets.unloadBundle(['background', 'characters']);
    };
  }, []);

  return (
    <div className="w-full h-full flex justify-center">
      <Application
        width={360}
        height={640}
        defaultTextStyle={{ fill: 0xffcb30 }}
      >
        <DevTool />
        {initialized && children}
      </Application>
    </div>
  );
}
