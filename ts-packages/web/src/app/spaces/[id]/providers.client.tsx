'use client';

import { DehydratedState, HydrationBoundary } from '@tanstack/react-query';
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
  return (
    <HydrationBoundary state={dehydratedState}>
      <Context.Provider value={{ spaceId }}>{children}</Context.Provider>
    </HydrationBoundary>
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
