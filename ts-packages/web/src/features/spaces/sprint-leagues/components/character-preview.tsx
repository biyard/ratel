import { useEffect, useRef, useState } from 'react';
import { PlayerImage, SpriteSheet } from '../types/sprint-league-player';

/**
 * CharacterPreview component for displaying sprite sheet animations in DOM
 * Used in selection modals and editors (not for Konva canvas)
 */
export default function CharacterPreview({
  spriteSheet,
  className = '',
}: {
  spriteSheet: SpriteSheet;
  className?: string;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [frameData, setFrameData] = useState<{
    frames: Array<{ x: number; y: number; w: number; h: number }>;
    width: number;
    height: number;
  } | null>(null);
  const animationFrameRef = useRef<number | undefined>(undefined);
  const frameIndexRef = useRef(0);

  // Load and parse the sprite sheet JSON
  useEffect(() => {
    if (!spriteSheet.json) return;

    fetch(spriteSheet.json)
      .then((response) => response.json())
      .then((data) => {
        const frames: Array<{ x: number; y: number; w: number; h: number }> =
          [];
        let width = 0;
        let height = 0;

        const sortedKeys = Object.keys(data.frames).sort();
        if (sortedKeys.length > 0) {
          const firstFrame = data.frames[sortedKeys[0]].frame;
          width = firstFrame.w;
          height = firstFrame.h;
        }

        for (const key of sortedKeys) {
          const frame = data.frames[key].frame;
          frames.push({
            x: frame.x,
            y: frame.y,
            w: frame.w,
            h: frame.h,
          });
        }

        setFrameData({ frames, width, height });
      })
      .catch((error) =>
        console.error('Error loading sprite sheet JSON:', error),
      );
  }, [spriteSheet.json]);

  // Animate the sprite
  useEffect(() => {
    if (!frameData || !spriteSheet.image) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const image = new Image();
    image.crossOrigin = 'anonymous';
    image.src = spriteSheet.image;

    let lastFrameTime = 0;
    const frameDelay = 100; // milliseconds per frame

    const animate = (currentTime: number) => {
      if (currentTime - lastFrameTime >= frameDelay) {
        const frame = frameData.frames[frameIndexRef.current];

        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw current frame
        ctx.drawImage(
          image,
          frame.x,
          frame.y,
          frame.w,
          frame.h,
          0,
          0,
          canvas.width,
          canvas.height,
        );

        // Move to next frame
        frameIndexRef.current =
          (frameIndexRef.current + 1) % frameData.frames.length;
        lastFrameTime = currentTime;
      }

      animationFrameRef.current = requestAnimationFrame(animate);
    };

    image.onload = () => {
      animationFrameRef.current = requestAnimationFrame(animate);
    };

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [frameData, spriteSheet.image]);

  if (!frameData) {
    return (
      <div
        className={`flex items-center justify-center bg-neutral-800 light:bg-neutral-100 ${className}`}
      >
        <span className="text-neutral-400">Loading...</span>
      </div>
    );
  }

  return (
    <canvas
      ref={canvasRef}
      width={frameData.width}
      height={frameData.height}
      className={`w-full h-full object-contain ${className}`}
      style={{ imageRendering: 'pixelated' }}
    />
  );
}

/**
 * Static character image preview (for win/lose states)
 */
export function CharacterImagePreview({
  imageUrl,
  alt = 'Character',
  className = '',
}: {
  imageUrl: string;
  alt?: string;
  className?: string;
}) {
  return (
    <img
      src={imageUrl}
      alt={alt}
      className={`w-full h-full object-contain ${className}`}
      style={{ imageRendering: 'pixelated' }}
    />
  );
}

/**
 * Full character preview with all states (for selection modal)
 */
export function FullCharacterPreview({
  images,
  className = '',
}: {
  images: PlayerImage;
  className?: string;
}) {
  return (
    <div className={`relative ${className}`}>
      <CharacterPreview spriteSheet={images.select} />
    </div>
  );
}
