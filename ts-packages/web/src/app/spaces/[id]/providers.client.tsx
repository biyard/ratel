'use client';

import { getQueryClient } from '@/providers/getQueryClient';
import {
  DehydratedState,
  HydrationBoundary,
  QueryClientProvider,
} from '@tanstack/react-query';
import React, { createContext, useContext } from 'react';

type ContextType = {
  spaceId: number;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  dehydratedState,
  spaceId,
}: {
  children: React.ReactNode;
  dehydratedState: DehydratedState;
  spaceId: number;
}) {
  const queryClient = getQueryClient();
  return (
    <QueryClientProvider client={queryClient}>
      <HydrationBoundary state={dehydratedState}>
        <Context.Provider value={{ spaceId }}>{children}</Context.Provider>
      </HydrationBoundary>
    </QueryClientProvider>
  );
}

export function useSpaceByIdContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
