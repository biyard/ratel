import { config } from '@/config';
import { ApolloClient, InMemoryCache } from '@apollo/client';
import { cache } from 'react';

export const client = new ApolloClient({
  uri: config.graphql_url,
  cache: new InMemoryCache(),
});

export const apolloServerClient = cache(
  () =>
    new ApolloClient({
      ssrMode: true,
      uri: config.graphql_url,
      cache: new InMemoryCache(),
    }),
);
