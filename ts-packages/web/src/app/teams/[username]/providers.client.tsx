'use client';

import { DehydratedState, HydrationBoundary } from '@tanstack/react-query';
import React, { createContext, useContext } from 'react';

type ContextType = {
  userId: number;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  dehydratedState,
  userId,
}: {
  children: React.ReactNode;
  dehydratedState: DehydratedState;
  userId: number;
}) {
  return (
    <HydrationBoundary state={dehydratedState}>
      <Context.Provider value={{ userId }}>{children}</Context.Provider>
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
