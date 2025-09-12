//// filepath: /Users/ryan/biyard/ratel/ts-packages/web/src/app/spaces/[id]/sprint-league/_components/konva/background.tsx
'use client';

import { useEffect, useRef, useState } from 'react';
import { Image as KonvaImage, Layer, Group } from 'react-konva';
import useImage from 'use-image';
import { IRect } from 'konva/lib/types';
import { VIEWPORT_HEIGHT, VIEWPORT_WIDTH } from '../';

const IMAGE_URL = '/images/sprint_league/background.png';

const LAYERS_CONFIG: {
  name: string;
  frame: IRect;
  speed?: number;
}[] = [
  { name: 'sky_gradient', frame: { x: 0, y: 0, width: 2880, height: 640 } },
  {
    name: 'sunset_clouds',
    frame: { x: 0, y: 640, width: 2880, height: 640 },
    speed: 0.2,
  },
  {
    name: 'distant_clouds',
    frame: { x: 0, y: 1280, width: 2880, height: 640 },
    speed: 0.3,
  },
  {
    name: 'ground_strip_1',
    frame: { x: 0, y: 1920, width: 2880, height: 640 },
    speed: 0.2,
  },
  {
    name: 'far_hills',
    frame: { x: 0, y: 2560, width: 2880, height: 640 },
    speed: 0.4,
  },
  {
    name: 'mid_hills',
    frame: { x: 0, y: 3200, width: 2880, height: 640 },
    speed: 0.45,
  },
  {
    name: 'green_line',
    frame: { x: 0, y: 3840, width: 2880, height: 640 },
    speed: 0.5,
  },
  {
    name: 'yellow_line',
    frame: { x: 0, y: 4480, width: 2880, height: 640 },
    speed: 0.7,
  },
  {
    name: 'trees_row_1',
    frame: { x: 0, y: 5120, width: 2880, height: 640 },
    speed: 0.8,
  },
  {
    name: 'trees_row_2',
    frame: { x: 0, y: 5760, width: 2880, height: 640 },
    speed: 0.8,
  },
  {
    name: 'grass_texture_1',
    frame: { x: 0, y: 6400, width: 2880, height: 640 },
    speed: 1,
  },
  {
    name: 'grass_texture_2',
    frame: { x: 0, y: 7050, width: 2880, height: 640 },
    speed: 1.4,
  },
  {
    name: 'grass_texture_3',
    frame: { x: 0, y: 7680, width: 2880, height: 640 },
    speed: 1.6,
  },
  {
    name: 'foreground_ground',
    frame: { x: 0, y: 8320, width: 2880, height: 640 },
  },
];

type SeamlessScrollerProps = {
  image: HTMLImageElement;
  frame: IRect;
  speedPxPerSec: number;
  y?: number;
};

function SeamlessScroller({
  image,
  frame,
  speedPxPerSec,
  y = 0,
}: SeamlessScrollerProps) {
  const [positions, setPositions] = useState([
    { x: 0, y },
    { x: frame.width, y },
  ]);
  const rafRef = useRef<number | null>(null);
  const lastTimeRef = useRef<number | null>(null);

  useEffect(() => {
    const loop = (time: number) => {
      if (lastTimeRef.current == null) lastTimeRef.current = time;
      const dt = (time - lastTimeRef.current) / 1000;
      lastTimeRef.current = time;

      const delta = speedPxPerSec * dt;

      setPositions((prev) => {
        let [a, b] = prev;
        a = { ...a, x: a.x - delta };
        b = { ...b, x: b.x - delta };

        if (a.x <= -frame.width) a.x = b.x + frame.width;
        if (b.x <= -frame.width) b.x = a.x + frame.width;

        return [a, b];
      });

      rafRef.current = requestAnimationFrame(loop);
    };
    rafRef.current = requestAnimationFrame(loop);
    return () => {
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [frame.width, speedPxPerSec]);

  const crop = {
    x: frame.x,
    y: frame.y,
    width: frame.width,
    height: frame.height,
  };

  return (
    <>
      {positions.map((pos, i) => (
        <KonvaImage
          key={i}
          image={image}
          x={pos.x}
          y={pos.y}
          crop={crop}
          width={frame.width}
          height={frame.height}
        />
      ))}
    </>
  );
}

export default function Background({
  baseSpeed = 1000,
}: {
  baseSpeed?: number;
}) {
  const [image] = useImage(IMAGE_URL);
  if (!image) return null;

  return (
    <Layer listening={false}>
      <Group
        x={0}
        y={0}
        clip={{
          x: 0,
          y: 0,
          width: VIEWPORT_WIDTH,
          height: VIEWPORT_HEIGHT,
        }}
      >
        {LAYERS_CONFIG.map((cfg) => (
          <SeamlessScroller
            key={cfg.name}
            image={image}
            frame={cfg.frame}
            speedPxPerSec={(cfg.speed || 0) * baseSpeed}
            y={0}
          />
        ))}
      </Group>
    </Layer>
  );
}
