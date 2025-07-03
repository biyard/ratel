'use client';

import { Follower } from '@/lib/api/models/network';
import { useNetwork } from '@/lib/api/ratel_api';
import React, { createContext, useContext } from 'react';

type ContextType = {
  suggestions: Follower[];
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data } = useNetwork();
  const suggestions = [
    ...(data?.suggested_teams.slice(0, 3) || []),
    ...(data?.suggested_users.slice(0, 3) || []),
  ].slice(0, 3);

  return (
    <Context.Provider value={{ suggestions }}>{children}</Context.Provider>
  );
}

export function useHomeContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}
