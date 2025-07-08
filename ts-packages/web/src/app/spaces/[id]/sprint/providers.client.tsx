'use client';

import React, { createContext, useContext } from 'react';

type ContextType = {
  spaceId: number;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  spaceId,
}: {
  children: React.ReactNode;
  spaceId: number;
}) {
  return <Context.Provider value={{ spaceId }}>{children}</Context.Provider>;
}

export function useSpaceByIdContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
