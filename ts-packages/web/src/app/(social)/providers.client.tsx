'use client';

import { makeClient } from '@/lib/apollo-client';
import { ApolloNextAppProvider } from '@apollo/client-integration-nextjs';
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
  return (
    <HydrationBoundary state={dehydratedState}>
      <ApolloNextAppProvider
        makeClient={() => makeClient(JSON.parse(apolloCache))}
      >
        {children}
      </ApolloNextAppProvider>
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
