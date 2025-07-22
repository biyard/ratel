'use client';
import React, { useEffect } from 'react';
import type { AssetsBundle } from 'pixi.js';

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

extend({
  Container,
  Graphics,
  Sprite,
  AnimatedSprite,
  Text,
});

export const SCALE = 3;

export const WIDTH = SCALE * 360;
export const HEIGHT = SCALE * 640;

function DevTool() {
  const { app } = useApplication();
  useEffect(() => {
    initDevtools({ app });
  }, [app]);
  return <></>;
}

export default function Base({
  children,
  bundles,
}: {
  children: React.ReactNode;
  bundles: AssetsBundle[];
}) {
  const [initialized, setInitialized] = React.useState(false);
  useEffect(() => {
    const names = bundles.map((bundle) => bundle.name);
    const loadAssets = async () => {
      try {
        for (const bundle of bundles) {
          if (!Assets.resolver.hasBundle(bundle.name)) {
            Assets.addBundle(bundle.name, bundle.assets);
          }
        }

        await Assets.loadBundle(names);

        setInitialized(true);
      } catch (error) {
        console.error('Asset Load Failed:', error);
      }
    };
    loadAssets();
  }, [bundles]);
  return (
    <div className="w-full h-full flex justify-center">
      <Application
        width={WIDTH}
        height={HEIGHT}
        defaultTextStyle={{ fill: 0xffcb30, fontFamily: 'Raleway' }}
      >
        <DevTool />
        {initialized && children}
      </Application>
    </div>
  );
}
