import { defineConfig } from 'orval';

export default defineConfig({
  api: {
    output: {
      mode: 'tags-split',
      // namingConvention: 'PascalCase',
      target: 'src/api/endpoints.ts',
      // schemas: 'src/api/model',
      client: 'react-query',
      httpClient: 'fetch',
      baseUrl: 'http://localhost:8000',
      mock: false,
      prettier: true,
      override: {
        mutator: {
          path: './src/api/mutator.ts',
          name: 'customFetch',
        },
      },
    },

    input: {
      target: 'http://localhost:4000/docs/api.json',
      override: {
        transformer: './orval-transformer.js',
      },

      //   filters: {
      //     paths: '/^\\/v3\\//',
      //   },
    },
  },
});
