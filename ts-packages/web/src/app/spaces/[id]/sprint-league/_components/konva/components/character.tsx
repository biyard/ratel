'use client';

import { useEffect, useRef, useState } from 'react';
import { Sprite } from 'react-konva';
import useImage from 'use-image';
import type { Sprite as KonvaSprite } from 'konva/lib/shapes/Sprite';
import { CHARACTER_SIZE } from '../';

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

export default function CharacterSprite({
  imageUrl,
  jsonUrl,
  x,
  y,
  scale,
  speed = 1,
}: {
  imageUrl: string;
  jsonUrl: string;
  x: number;
  y: number;
  scale: number;
  speed?: number;
}) {
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
      if (speed > 0) {
        spriteRef.current.start();
      } else {
        spriteRef.current.stop();
      }
    }
  }, [speed, animation, image]);

  if (!image || !animation) {
    return null;
  }
  const frameScale = (CHARACTER_SIZE / (frameSize ? frameSize[0] : 1)) * scale;
  return (
    <Sprite
      ref={spriteRef}
      scale={{ x: frameScale, y: frameScale }}
      animation={'run'}
      animations={{
        run: animation,
      }}
      frameRate={10 * speed}
      frameIndex={0}
      offsetX={CHARACTER_SIZE / 2}
      offsetY={CHARACTER_SIZE / 2}
      image={image}
      x={x}
      y={y}
    />
  );
}
