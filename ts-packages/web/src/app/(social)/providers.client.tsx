'use client';

import { makeClient } from '@/lib/apollo-client';
import { ApolloNextAppProvider } from '@apollo/client-integration-nextjs';
import React, { ReactNode, createContext, useContext } from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  apolloCache,
}: {
  children: ReactNode;
  apolloCache: string;
}) {
  return (
    <ApolloNextAppProvider
      makeClient={() => makeClient(JSON.parse(apolloCache))}
    >
      {children}
    </ApolloNextAppProvider>
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
