'use client';

import { Assets, AssetsBundle, Spritesheet } from 'pixi.js';
import { PlayerImages } from '../player';

class PixiAssetManager {
  private static instance: PixiAssetManager;
  private loadingPromises = new Map<string, Promise<Spritesheet>>();
  private loadingBundles = new Map<string, Promise<void>>();
  private loadedAssets = new Set<string>();

  static getInstance() {
    if (!PixiAssetManager.instance) {
      PixiAssetManager.instance = new PixiAssetManager();
    }
    return PixiAssetManager.instance;
  }

  async getAsset(cacheKey: string): Promise<Spritesheet | null> {
    if (this.loadedAssets.has(cacheKey)) {
      return Assets.cache.get(cacheKey) as Spritesheet;
    }
    if (Assets.cache.has(cacheKey)) {
      this.loadedAssets.add(cacheKey);
      return Assets.cache.get(cacheKey) as Spritesheet;
    }

    return null;
  }

  async loadPlayerAsset(assets: PlayerImages): Promise<void> {
    await this.unloadBundle(`${assets.alias}_bundle`);
    const bundles: AssetsBundle = {
      name: `${assets.alias}_bundle`,
      assets: [
        { alias: `${assets.alias}_run`, src: assets.run.json },
        { alias: `${assets.alias}_selected`, src: assets.select.json },
        { alias: `${assets.alias}_win`, src: assets.win },
        { alias: `${assets.alias}_lose`, src: assets.lose },
      ],
    };

    await this.loadBundle(bundles);
  }

  async loadSpritesheet(
    cacheKey: string,
    jsonUrl: string,
  ): Promise<Spritesheet> {
    if (this.loadingPromises.has(cacheKey)) {
      return this.loadingPromises.get(cacheKey)!;
    }

    if (Assets.cache.has(cacheKey)) {
      this.loadedAssets.add(cacheKey);
      return Assets.cache.get(cacheKey) as Spritesheet;
    }

    const loadPromise = this.loadAsset<Spritesheet>(cacheKey, jsonUrl);
    this.loadingPromises.set(cacheKey, loadPromise);

    try {
      const sheet = await loadPromise;
      this.loadingPromises.delete(cacheKey);
      this.loadedAssets.add(cacheKey);
      return sheet;
    } catch (error) {
      this.loadingPromises.delete(cacheKey);
      throw error;
    }
  }

  async loadBundle(bundle: AssetsBundle): Promise<void> {
    const cacheKey = bundle.name;
    if (!cacheKey) {
      return;
    }
    if (this.loadingBundles.has(cacheKey)) {
      return this.loadingBundles.get(cacheKey)!;
    }

    const loadPromise = (async () => {
      Assets.addBundle(cacheKey, bundle.assets);
      await Assets.loadBundle(cacheKey);
    })();

    this.loadingBundles.set(cacheKey, loadPromise);

    try {
      await loadPromise;
    } catch (error) {
      throw error;
    } finally {
      this.loadingBundles.delete(cacheKey);
    }
  }
  private async loadAsset<T = Spritesheet>(
    alias: string,
    jsonUrl: string,
  ): Promise<T> {
    try {
      const sheet = await Assets.load({
        alias,
        src: jsonUrl,
      });
      if (sheet instanceof Spritesheet) {
        if (
          !sheet ||
          !sheet.textures ||
          Object.keys(sheet.textures).length === 0
        ) {
          throw new Error(`Invalid spritesheet: ${jsonUrl}`);
        }
      }

      return sheet;
    } catch (error) {
      console.error(`Failed to load asset ${alias}:`, error);
      throw error;
    }
  }

  async unloadBundle(bundleName: string) {
    await Assets.unloadBundle(bundleName);
    this.loadingBundles.delete(bundleName);
  }

  unloadAsset(cacheKey: string) {
    if (Assets.cache.has(cacheKey)) {
      Assets.unload(cacheKey);
    }

    this.loadedAssets.delete(cacheKey);
    this.loadingPromises.delete(cacheKey);
  }

  unloadAll() {
    for (const cacheKey of this.loadedAssets) {
      if (Assets.cache.has(cacheKey)) {
        Assets.unload(cacheKey);
      }
    }

    this.loadedAssets.clear();
    this.loadingPromises.clear();
    this.loadingBundles.clear();
  }

  isLoading(cacheKey: string): boolean {
    return this.loadingPromises.has(cacheKey);
  }

  isLoaded(cacheKey: string): boolean {
    return this.loadedAssets.has(cacheKey) || Assets.cache.has(cacheKey);
  }
}

export const pixiAssetManager = PixiAssetManager.getInstance();
