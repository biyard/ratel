import { useEffect, useRef, useState } from 'react';
import { SpriteSheet } from '../types/sprint-league-player';

interface FrameData {
  frame: { x: number; y: number; w: number; h: number };
  duration: number;
}

interface SpriteData {
  frames: Record<string, FrameData>;
}

/**
 * Character component displays an animated sprite preview
 * Used in the editor to show the selected character with animation
 */
export default function Character({
  spriteSheet,
}: {
  spriteSheet: SpriteSheet;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [spriteData, setSpriteData] = useState<SpriteData | null>(null);
  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const currentFrameIndexRef = useRef(0);
  const lastFrameTimeRef = useRef(0);

  // Load sprite data from JSON
  useEffect(() => {
    if (!spriteSheet.json) return;

    fetch(spriteSheet.json)
      .then((response) => response.json())
      .then((data: SpriteData) => {
        setSpriteData(data);
      })
      .catch((error) => console.error('Error loading sprite data:', error));
  }, [spriteSheet.json]);

  // Load sprite image
  useEffect(() => {
    if (!spriteSheet.image) return;

    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => setImage(img);
    img.onerror = (error) =>
      console.error('Error loading sprite image:', error);
    img.src = spriteSheet.image;

    return () => {
      img.onload = null;
      img.onerror = null;
    };
  }, [spriteSheet.image]);

  // Animation loop
  useEffect(() => {
    if (!canvasRef.current || !spriteData || !image) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const frames = Object.keys(spriteData.frames).sort();
    if (frames.length === 0) return;

    const animate = (timestamp: number) => {
      if (!lastFrameTimeRef.current) {
        lastFrameTimeRef.current = timestamp;
      }

      const elapsed = timestamp - lastFrameTimeRef.current;
      const frameRate = 100; // milliseconds per frame (10 FPS)

      if (elapsed > frameRate) {
        lastFrameTimeRef.current = timestamp;

        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Get current frame data
        const frameKey = frames[currentFrameIndexRef.current];
        const frameData = spriteData.frames[frameKey].frame;

        // Calculate scale to fit canvas
        const scaleX = canvas.width / frameData.w;
        const scaleY = canvas.height / frameData.h;
        const scale = Math.min(scaleX, scaleY);

        // Calculate centered position
        const scaledWidth = frameData.w * scale;
        const scaledHeight = frameData.h * scale;
        const x = (canvas.width - scaledWidth) / 2;
        const y = (canvas.height - scaledHeight) / 2;

        // Draw current frame
        ctx.drawImage(
          image,
          frameData.x,
          frameData.y,
          frameData.w,
          frameData.h,
          x,
          y,
          scaledWidth,
          scaledHeight,
        );

        // Move to next frame
        currentFrameIndexRef.current =
          (currentFrameIndexRef.current + 1) % frames.length;
      }

      animationFrameRef.current = requestAnimationFrame(animate);
    };

    animationFrameRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [spriteData, image]);

  return (
    <canvas
      ref={canvasRef}
      width={300}
      height={300}
      className="w-full h-full"
      style={{ imageRendering: 'pixelated' }}
    />
  );
}
