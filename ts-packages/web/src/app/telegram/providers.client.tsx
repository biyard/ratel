'use client';

import React, { ReactNode, createContext, useContext } from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({ children }: { children: ReactNode }) {
  return <Context.Provider value={{}}>{children}</Context.Provider>;
}

export function useTelegramContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
