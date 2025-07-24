'use client';

import { ApolloClient, ApolloProvider, InMemoryCache } from '@apollo/client';
import {
  QueryClientProvider,
  HydrationBoundary,
  DehydratedState,
} from '@tanstack/react-query';
import React, {
  ReactNode,
  createContext,
  useContext,
  useMemo,
  useState,
  useEffect,
} from 'react';
import { getQueryClient } from '@/providers/getQueryClient';
import { Follower } from '@/lib/api/models/network';
import { useNetwork } from '@/lib/api/ratel_api';

type ContextType = {
  suggestions: Follower[];
};

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
  const queryClient = getQueryClient();

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
    <QueryClientProvider client={queryClient}>
      <HydrationBoundary state={dehydratedState}>
        <ApolloProvider client={apolloClient}>
          <NetworkContextWrapper>{children}</NetworkContextWrapper>
        </ApolloProvider>
      </HydrationBoundary>
    </QueryClientProvider>
  );
}

function NetworkContextWrapper({ children }: { children: ReactNode }) {
  const { data } = useNetwork();

  const suggestions = useMemo(() => {
    return [
      ...(data?.suggested_teams.slice(0, 3) || []),
      ...(data?.suggested_users.slice(0, 3) || []),
    ].slice(0, 3);
  }, [data]);

  return (
    <Context.Provider value={{ suggestions }}>{children}</Context.Provider>
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
