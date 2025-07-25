'use client';

import { ApolloClient, ApolloProvider, InMemoryCache } from '@apollo/client';
import { HydrationBoundary, DehydratedState } from '@tanstack/react-query';
import React, {
  ReactNode,
  createContext,
  useContext,
  useState,
  useEffect,
} from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  dehydratedState,
  apolloCache,
}: {
  children: ReactNode;
  dehydratedState: DehydratedState;
  apolloCache: string;
}) {
  const [apolloClient] = useState(() => {
    const cache = new InMemoryCache();
    cache.restore(JSON.parse(apolloCache));
    return new ApolloClient({
      uri: process.env.NEXT_PUBLIC_GRAPHQL_URL!,
      ssrMode: true,
      cache,
    });
  });

  const [isApolloReady, setApolloReady] = useState(false);

  useEffect(() => {
    setApolloReady(true);
  }, []);

  if (!isApolloReady) return null;

  return (
    <HydrationBoundary state={dehydratedState}>
      <ApolloProvider client={apolloClient}>{children}</ApolloProvider>
    </HydrationBoundary>
  );
}

export function useHomeContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}
