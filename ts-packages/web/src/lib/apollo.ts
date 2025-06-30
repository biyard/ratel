import { config } from '@/config';
import { ApolloClient, InMemoryCache } from '@apollo/client';

export const client = new ApolloClient({
  uri: config.graphql_url,
  cache: new InMemoryCache(),
});
