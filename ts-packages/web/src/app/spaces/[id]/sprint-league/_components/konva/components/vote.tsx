'use client';

import { Group, Image as KonvaImage } from 'react-konva';
import useImage from 'use-image';
import { useEffect, useRef, useMemo } from 'react';
import type { Group as KonvaGroupType } from 'konva/lib/Group';
import type { Image as KonvaImageType } from 'konva/lib/shapes/Image';
import CharacterSprite from './character';

const VOTE_SELECTED = '/images/sprint_league/vote_selected.png';
const VOTE_UNSELECTED = '/images/sprint_league/vote_unselected.png';
const VOTE_DEFAULT = '/images/sprint_league/vote_default.png';

function centerOffset(img?: HTMLImageElement | null) {
  if (!img) return { offsetX: 0, offsetY: 0 };
  return { offsetX: img.width / 2, offsetY: img.height / 2 };
}

interface VoteItemProps {
  imageUrl: string;
  jsonUrl: string;
  isSelected: boolean | null;
  onClick: () => void;
  x: number;
  y: number;
}

export default function VoteItem({
  jsonUrl,
  imageUrl,
  isSelected,
  onClick,
  x,
  y,
}: VoteItemProps) {
  const groupRef = useRef<KonvaGroupType>(null);
  const imgRef = useRef<KonvaImageType>(null);

  const [imgDefault] = useImage(VOTE_DEFAULT);
  const [imgSelected] = useImage(VOTE_SELECTED);
  const [imgUnselected] = useImage(VOTE_UNSELECTED);

  const imageToUse = useMemo(() => {
    if (isSelected === true) return imgSelected || imgDefault;
    if (isSelected === false) return imgUnselected || imgDefault;
    return imgDefault;
  }, [isSelected, imgDefault, imgSelected, imgUnselected]);

  useEffect(() => {
    let frameId: number;
    let last = performance.now();
    const baseY = y;
    const targetOffset = isSelected ? -20 : 0;

    const loop = (t: number) => {
      const dt = (t - last) / 1000;
      last = t;
      const g = groupRef.current;
      if (g) {
        const desiredY = baseY + targetOffset;
        const currentY = g.y();
        const nextY = currentY + (desiredY - currentY) * Math.min(1, 6 * dt);
        g.y(nextY);
      }
      frameId = requestAnimationFrame(loop);
    };
    frameId = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(frameId);
  }, [y, isSelected]);

  if (!imageToUse) return null;

  const { offsetX, offsetY } = centerOffset(imageToUse);

  return (
    <Group ref={groupRef} onClick={onClick} x={x}>
      <KonvaImage
        ref={imgRef}
        image={imageToUse}
        x={0}
        y={0}
        offsetX={offsetX}
        offsetY={offsetY}
        listening={false}
      />
      <Group
        clip={{ x: -40, y: -100, width: 80, height: 200 }}
        listening={true}
      >
        <CharacterSprite
          x={-45}
          y={-30}
          scale={0.9}
          speed={0}
          imageUrl={imageUrl}
          jsonUrl={jsonUrl}
        />
      </Group>
    </Group>
  );
}
