'use client';

// TODO: Remove Apollo Client after full migration to REST v2
// @deprecated - Migrating to REST v2
// import { makeClient } from '@/lib/apollo-client';
// import { ApolloNextAppProvider } from '@apollo/client-integration-nextjs';
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
  // TODO: Remove Apollo provider after full migration to REST v2
  // @deprecated - Migrating to REST v2
  // return (
  //   <ApolloNextAppProvider
  //     makeClient={() => makeClient(JSON.parse(apolloCache))}
  //   >
  //     <Context.Provider value={{}}>{children}</Context.Provider>
  //   </ApolloNextAppProvider>
  // );
  
  // Temporary wrapper during migration
  return <Context.Provider value={{}}>{children}</Context.Provider>;
}

export function useHomeContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}
