import { defineConfig } from 'orval';

export default defineConfig({
  api: {
    output: {
      mode: 'tags-split',
      // namingConvention: 'PascalCase',
      target: 'src/api/endpoints.ts',
      client: 'fetch',
      httpClient: 'fetch',
      //FIXME: Use env variable
      baseUrl: 'http://localhost:4000', // API base URL
      mock: false,
      prettier: true,
      override: {
        mutator: {
          path: './src/api/fetch.ts', // Custom Fetch function
          name: 'call', // Function name
        },
      },
      urlEncodeParameters: true,
    },

    input: {
      target: 'http://localhost:4000/docs/api.json', // OpenAPI spec URL
      override: {
        transformer: './orval-transformer.js', // Filter Paths and Remove unused schemas
      },
    },
  },
});
