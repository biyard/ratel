'use client';

import { createContext, useContext } from 'react';

type ContextType = object;

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  return <Context.Provider value={{}}>{children}</Context.Provider>;
}

export function useSettingsContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
