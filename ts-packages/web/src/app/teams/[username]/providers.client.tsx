'use client';

import { createContext, useContext } from 'react';

type ContextType = {
  userId: number;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  userId,
}: {
  children: React.ReactNode;
  userId: number;
}) {
  return <Context.Provider value={{ userId }}>{children}</Context.Provider>;
}

export function useTeamsContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}
