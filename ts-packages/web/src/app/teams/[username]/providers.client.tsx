'use client';

import { createContext } from 'react';

type ContextType = {
  userId: number;
};

const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  userId,
}: {
  children: React.ReactNode;
  userId: number;
}) {
  return <Context.Provider value={{ userId }}>{children}</Context.Provider>;
}
