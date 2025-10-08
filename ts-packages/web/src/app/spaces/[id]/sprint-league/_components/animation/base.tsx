// @ts-nocheck
'use client';
import * as React from 'react';

import { useEffect } from 'react';
import type { AssetsBundle } from 'pixi.js';

import { Container, AnimatedSprite, Graphics, Sprite, Text } from 'pixi.js';
import { extend, Application, useApplication } from '@pixi/react';
import { initDevtools } from '@pixi/devtools';
import { pixiAssetManager } from './assets';

extend({
  Container,
  Graphics,
  Sprite,
  AnimatedSprite,
  Text,
});

function DevTool() {
  const { app } = useApplication();
  useEffect(() => {
    initDevtools({ app });
  }, [app]);
  return <></>;
}

export default function Base({
  ref,
  children,
  bundles,
}: {
  ref: React.RefObject<HTMLDivElement | null>;
  children: React.ReactNode;
  bundles: AssetsBundle[];
}) {
  // const height = (width * 640) / 360;
  const [initialized, setInitialized] = React.useState(false);

  useEffect(() => {
    const loadAssets = async () => {
      try {
        const promises = bundles.map((bundle) =>
          pixiAssetManager.loadBundle(bundle),
        );
        await Promise.all(promises);
        setInitialized(true);
      } catch (error) {
        console.error('Asset Load Failed:', error);
      }
    };
    loadAssets();
  }, [bundles]);

  return (
    <Application
      clearBeforeRender
      resizeTo={ref}
      defaultTextStyle={{ fill: 0xffcb30, fontFamily: 'Raleway' }}
    >
      {process.env.NEXT_PUBLIC_ENV === 'development' && <DevTool />}
      {initialized && children}
    </Application>
  );
}
