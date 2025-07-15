'use client';
// import React, { useEffect, useRef, useState } from 'react';
// import type { AssetsBundle, Spritesheet, Texture } from 'pixi.js';

import {
  Container,
  Assets,
  Text,
  Texture,
  TilingSprite,
  Spritesheet,
} from 'pixi.js';
import { extend, useApplication, useTick } from '@pixi/react';
import { useEffect, useRef, useState } from 'react';
import { SCALE } from './base';

extend({
  Container,
  Text,
  TilingSprite,
});

const LAYERS_CONFIG: { name: string; speed?: number; scale?: number }[] = [
  { name: 'sky_gradient' },
  { name: 'sunset_clouds', speed: 0.2 },
  { name: 'distant_clouds', speed: 0.3 },
  { name: 'ground_strip_1', speed: 0.2 },
  { name: 'far_hills', speed: 0.4 },
  { name: 'mid_hills', speed: 0.45 },
  { name: 'green_line', speed: 0.5 },
  { name: 'yellow_line', speed: 0.7 },
  { name: 'trees_row_1', speed: 0.8 },
  { name: 'trees_row_2', speed: 0.8 },
  { name: 'grass_texture_1', speed: 1 },
  { name: 'grass_texture_2', speed: 1.4 },
  { name: 'grass_texture_3', speed: 1.6 },
];

const ScrollingLayer = ({
  texture,
  y = 0,
  speed = 0,
  scale = SCALE,
}: {
  texture: Texture;
  y?: number;
  speed?: number;
  scale?: number;
}) => {
  const spriteRef = useRef<TilingSprite>(null);
  const { app } = useApplication();

  useTick((ticker) => {
    if (spriteRef.current) {
      spriteRef.current.tilePosition.x -= speed * ticker.deltaTime;
    }
  });

  return (
    <pixiTilingSprite
      ref={spriteRef}
      texture={texture}
      y={y}
      scale={scale}
      width={app.renderer?.width}
      height={texture.height}
    />
  );
};
export function Dim() {
  const { app } = useApplication();
  return (
    <pixiSprite
      texture={Assets.get('foreground_ground')}
      width={app.renderer?.width}
      height={app.renderer?.height}
    />
  );
}
export default function Background({
  alias,
  baseSpeed = 1.5,
}: {
  alias: string;
  baseSpeed?: number;
}) {
  const [textures, setTextures] = useState<Record<string, Texture[]>>({});

  useEffect(() => {
    const loadAssets = async () => {
      try {
        const sheet = (await Assets.get(alias)) as Spritesheet;
        const texturesObj: Record<string, Texture[]> = {};
        Object.entries(sheet.textures).forEach(([key, tex]) => {
          texturesObj[key] = [tex];
        });
        console.log('Asset Loaded:', texturesObj);
        // Set the textures state
        setTextures(texturesObj);
      } catch (error) {
        console.error('Asset Load Failed:', error);
      }
    };

    loadAssets();
  }, [alias]);
  return (
    <pixiContainer>
      {LAYERS_CONFIG.map((layer) => {
        const texture = textures[layer.name]?.[0];
        if (!texture) return null;
        return (
          <ScrollingLayer
            key={layer.name}
            texture={texture}
            speed={(layer.speed || 0) * baseSpeed}
          />
        );
      })}
    </pixiContainer>
  );
}
