'use client';

import { useEffect, useRef, useState } from 'react';
import { Layer, Sprite, Stage } from 'react-konva';
import useImage from 'use-image';
import type { Sprite as KonvaSprite } from 'konva/lib/shapes/Sprite';
import Background from './background';
/*
const spriteData = {
  "frames": {
    "lee_jae_frame_0": { "frame": { "x": 1, "y": 1, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_1": { "frame": { "x": 278, "y": 1, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_2": { "frame": { "x": 555, "y": 1, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_3": { "frame": { "x": 832, "y": 1, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_4": { "frame": { "x": 1, "y": 278, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_5": { "frame": { "x": 278, "y": 278, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_6": { "frame": { "x": 555, "y": 278, "w": 276, "h": 276 }, "duration": 100 },
    "lee_jae_frame_7": { "frame": { "x": 832, "y": 278, "w": 276, "h": 276 }, "duration": 100 }
  },
  "meta": {
    "image": "/lee_jae_run.webp",
    "size": { "w": 1109, "h": 555 }
  }
};

*/

const parseFrames = (
  frames: Record<
    string,
    { frame: { x: number; y: number; w: number; h: number }; duration: number }
  >,
) => {
  let width = 0;
  let height = 0;

  const frameArray = [];
  const sortedKeys = Object.keys(frames).sort();
  // Get Frame Size from first frame
  if (sortedKeys.length > 0) {
    const firstFrame = frames[sortedKeys[0]].frame;
    width = firstFrame.w;
    height = firstFrame.h;
  }
  for (const key of sortedKeys) {
    const frameData = frames[key].frame;
    frameArray.push(frameData.x, frameData.y, frameData.w, frameData.h);
  }
  return { frameArray, width, height };
};

export function CharacterSprite({
  imageUrl,
  jsonUrl,
  ...props
}: { imageUrl: string; jsonUrl: string } & Omit<
  React.ComponentProps<typeof import('react-konva').Sprite>,
  'image'
>) {
  const spriteRef = useRef<KonvaSprite | null>(null);
  const [image] = useImage(imageUrl);

  const [animation, setAnimation] = useState<number[] | null>(null);
  const [frameSize, setFrameSize] = useState<[number, number] | null>(null);

  useEffect(() => {
    fetch(jsonUrl)
      .then((response) => response.json())
      .then((data) => {
        const { frameArray: frames, width, height } = parseFrames(data.frames);
        setFrameSize([width, height]);
        setAnimation(frames);
      })
      .catch((error) => console.error('Error fetching sprite data:', error));
  }, [jsonUrl]);

  useEffect(() => {
    if (spriteRef.current) {
      spriteRef.current.start();
    }
  }, [animation, image]);
  if (!image || !animation) {
    return null;
  }

  const scale = 150 / (frameSize ? frameSize[0] : 1);
  return (
    <Sprite
      ref={spriteRef}
      scale={{ x: scale, y: scale }}
      animation={'run'}
      animations={{
        run: animation,
      }}
      frameRate={10}
      frameIndex={0}
      image={image}
      {...props}
    />
  );
}

export default function CanvasStage({
  // children,
  width = 360,
  height = 640,
}: {
  // children: React.ReactNode;
  width?: number;
  height?: number;
}) {
  return (
    <Stage width={width} height={height}>
      <Background baseSpeed={1.5} />
      <Layer>
        <CharacterSprite
          imageUrl="/images/sprint_league/kim_moon_run.webp"
          jsonUrl="/images/sprint_league/kim_moon_run.json"
        />
        <CharacterSprite
          x={50}
          y={50}
          imageUrl="/images/sprint_league/lee_jae_run.webp"
          jsonUrl="/images/sprint_league/lee_jae_run.json"
        />
        <CharacterSprite
          x={100}
          y={100}
          imageUrl="/images/sprint_league/lee_jun_run.webp"
          jsonUrl="/images/sprint_league/lee_jun_run.json"
        />
      </Layer>
    </Stage>
  );
}
