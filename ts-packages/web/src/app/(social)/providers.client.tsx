'use client';

import { initializeApollo } from '@/lib/apollo-client';
import { ApolloProvider } from '@apollo/client';
import { HydrationBoundary, DehydratedState } from '@tanstack/react-query';
import React, { ReactNode, createContext, useContext } from 'react';

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
  const apolloClient = initializeApollo(JSON.parse(apolloCache));

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
