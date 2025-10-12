import type { StorybookConfig } from '@storybook/react-vite';
import svgr from 'vite-plugin-svgr';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';

const config: StorybookConfig = {
  stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: [
    '@chromatic-com/storybook',
    '@storybook/addon-docs',
    '@storybook/addon-onboarding',
    '@storybook/addon-a11y',
    '@storybook/addon-vitest',
  ],
  framework: {
    name: '@storybook/react-vite',
    options: {},
  },
  viteFinal: async (viteConfig) => {
    viteConfig.plugins = (viteConfig.plugins ?? []).concat([
      tailwindcss(),
      svgr({
        // optional SVGR options
        svgrOptions: {
          icon: true,
          // Example SVGO tweak: keep viewBox, remove width/height
          svgoConfig: {
            plugins: [
              { name: 'removeViewBox', active: false },
              'removeDimensions',
            ],
          },
        },
        // Export as React component with ?react OR default for all .svg
        include: '**/*.svg',
      }),
      react(),
    ]);
    return viteConfig;
  },
};
export default config;
