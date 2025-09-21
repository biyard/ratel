'use client';

import React, { ReactNode, createContext, useContext } from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

type ClientProvidersProps = {
  children: ReactNode;
};

export default function ClientProviders({ children }: ClientProvidersProps) {
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
