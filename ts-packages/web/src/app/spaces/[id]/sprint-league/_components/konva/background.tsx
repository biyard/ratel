'use client';

import { useEffect, useRef } from 'react';
import { Layer, Sprite } from 'react-konva';
import useImage from 'use-image';
import type { Layer as KonvaLayer } from 'konva/lib/Layer';
import type { Sprite as KonvaSprite } from 'konva/lib/shapes/Sprite';
const IMAGE_URL = '/images/sprint_league/background.png';

const konvaSpriteAnimations = {
  sky_gradient: [0, 0, 2880, 640],
  sunset_clouds: [0, 640, 2880, 640],
  distant_clouds: [0, 1280, 2880, 640],
  ground_strip_1: [0, 1920, 2880, 640],
  far_hills: [0, 2560, 2880, 640],
  mid_hills: [0, 3200, 2880, 640],
  green_line: [0, 3840, 2880, 640],
  yellow_line: [0, 4480, 2880, 640],
  trees_row_1: [0, 5120, 2880, 640],
  trees_row_2: [0, 5760, 2880, 640],
  grass_texture_1: [0, 6400, 2880, 640],
  grass_texture_2: [0, 7050, 2880, 640],
  grass_texture_3: [0, 7680, 2880, 640],
  foreground_ground: [0, 8320, 2880, 640],
};

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

export default function Background({
  baseSpeed = 1.5,
}: {
  baseSpeed?: number;
}) {
  const layerRef = useRef<KonvaLayer | null>(null);
  const [image] = useImage(IMAGE_URL);

  useEffect(() => {
    if (layerRef.current) {
      layerRef.current.find('Sprite').forEach((sprite) => {
        (sprite as KonvaSprite).start();
      });
    }
  }, [image]);

  if (!image) {
    return null;
  }

  return (
    <Layer ref={layerRef}>
      {LAYERS_CONFIG.map((layer) => {
        return (
          <Sprite
            key={layer.name}
            image={image}
            animations={konvaSpriteAnimations}
            animation={layer.name}
            speed={(layer.speed || 0) * baseSpeed}
          />
        );
      })}
    </Layer>
  );
}
