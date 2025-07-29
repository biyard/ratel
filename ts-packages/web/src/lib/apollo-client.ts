import { HttpLink, NormalizedCacheObject } from '@apollo/client';

import { ApolloClient, InMemoryCache } from '@apollo/client-integration-nextjs';

export function makeClient(initialCache?: NormalizedCacheObject) {
  const cache = new InMemoryCache();
  if (initialCache) {
    cache.restore(initialCache);
  }

  return new ApolloClient({
    cache,
    link: new HttpLink({
      uri: process.env.NEXT_PUBLIC_GRAPHQL_URL,
      credentials: 'include',
    }),
  });
}
