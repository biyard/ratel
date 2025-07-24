'use client';

import { getQueryClient } from '@/providers/getQueryClient';
import {
  DehydratedState,
  HydrationBoundary,
  QueryClientProvider,
} from '@tanstack/react-query';
import React, { ReactNode, createContext, useContext } from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  dehydratedState,
}: {
  children: ReactNode;
  dehydratedState: DehydratedState;
}) {
  const queryClient = getQueryClient();

  return (
    <QueryClientProvider client={queryClient}>
      <HydrationBoundary state={dehydratedState}>
        <Context.Provider value={{}}>{children}</Context.Provider>
      </HydrationBoundary>
    </QueryClientProvider>
  );
}

export function useTelegramContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
