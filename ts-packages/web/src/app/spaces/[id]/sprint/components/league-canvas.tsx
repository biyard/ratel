'use client';

import { useEffect, useRef } from 'react';
import { Application, Texture, AnimatedSprite } from 'pixi.js';

export default function SprintLeagueCanvas({
  width,
  height,
  targetPercents,
}: {
  width: number;
  height: number;
  targetPercents: number[];
}) {
  const canvasRef = useRef<HTMLDivElement>(null);
  const appRef = useRef<Application | null>(null);

  useEffect(() => {
    async function setup() {
      const app = new Application({
        width,
        height,
        antialias: true,
        autoDensity: true,
        resolution: window.devicePixelRatio,
      });

      await app.init({ backgroundAlpha: 0 });

      console.log('Canvas width:', app.canvas.width);
      console.log('Renderer width:', app.renderer.width);

      appRef.current = app;

      if (canvasRef.current) {
        canvasRef.current.innerHTML = '';
        canvasRef.current.appendChild(app.canvas);

        Object.assign(app.canvas.style, {
          position: 'absolute',
          top: '0',
          left: '0',
          width: '100%',
          height: '100%',
          backgroundColor: 'transparent',
          zIndex: '20',
          pointerEvents: 'none',
        });
      }

      const loadFrames = (
        name: string,
        frameCount: number,
      ): Promise<Texture[]> => {
        return new Promise((resolve) => {
          const frames: Texture[] = [];
          let loaded = 0;

          for (let i = 1; i <= frameCount; i++) {
            const index = String(i).padStart(3, '0');
            const src = `/images/${name}-frame-${index}.png`;

            const image = new Image();
            image.src = src;
            image.onload = () => {
              const texture = Texture.from(image);
              frames[i - 1] = texture;
              loaded++;
              if (loaded === frameCount) resolve(frames);
            };
            image.onerror = () => {
              console.error(`Failed to load frame: ${src}`);
            };
          }
        });
      };

      const runners = await Promise.all([
        loadFrames('lee-jae', 6),
        loadFrames('lee-jun', 6),
        loadFrames('kim-moon', 6),
      ]);

      const laneYs = [300, 370, 470];

      const sprites = [
        { frames: runners[0], lane: 0, speed: 2.0 },
        { frames: runners[1], lane: 1, speed: 1.5 },
        { frames: runners[2], lane: 2, speed: 1.2 },
      ].map(({ frames, lane, speed }, index) => {
        const sprite = new AnimatedSprite(frames);
        sprite.animationSpeed = 0.2;
        sprite.loop = true;
        sprite.play();
        sprite.x = 0;
        sprite.width = 150;
        sprite.height = 150;
        sprite.y = laneYs[lane];

        const targetX = (width / 3) * targetPercents[index];
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (sprite as any).targetX = targetX;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (sprite as any).speed = speed;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (sprite as any).stopped = false;

        app.stage.addChild(sprite);
        return sprite;
      });

      app.ticker.add(() => {
        sprites.forEach((sprite) => {
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const s = sprite as any;

          if (s.stopped) return;

          s.x += s.speed;

          if (s.x >= s.targetX) {
            s.x = s.targetX;
            s.speed = 0;
            s.stopped = true;
          }
        });
      });
    }

    setup();

    return () => {
      if (appRef.current) {
        appRef.current.destroy(true);
        appRef.current = null;
      }
    };
  }, [height]);

  return (
    <div
      ref={canvasRef}
      className="absolute inset-0 z-20 pointer-events-none bg-transparent"
      style={{ backgroundColor: 'transparent' }}
    />
  );
}
