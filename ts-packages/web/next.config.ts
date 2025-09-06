import type { NextConfig } from 'next';
import createNextIntlPlugin from 'next-intl/plugin';

const withNextIntl = createNextIntlPlugin();

const nextConfig: NextConfig = {
  eslint: {
    ignoreDuringBuilds: false,
  },
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: '**',
      },
    ],
  },
  turbopack: {
    rules: {
      '*.svg': {
        loaders: [
          {
            loader: '@svgr/webpack',
            options: {
              svgo: false,
              typescript: true,
              ext: 'tsx',
            },
          },
        ],
        as: '*.js',
      },
    },
  },

  webpack: (config) => {
    const fileLoaderRule = config.module.rules.find(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (rule: { test: { test: (arg0: string) => any } }) =>
        rule.test?.test?.('.svg'),
    );

    config.module.rules.push(
      {
        ...fileLoaderRule,
        test: /\.svg$/i,
        resourceQuery: /url/,
      },
      {
        test: /\.svg$/i,
        issuer: fileLoaderRule.issuer,
        resourceQuery: { not: [...fileLoaderRule.resourceQuery.not, /url/] },
        use: [
          {
            loader: '@svgr/webpack',
            options: {
              svgo: false,
              typescript: true,
              ext: 'tsx',
            },
          },
        ],
      },
    );
    fileLoaderRule.exclude = /\.svg$/i;
    return config;
  },
};

export default withNextIntl(nextConfig);
