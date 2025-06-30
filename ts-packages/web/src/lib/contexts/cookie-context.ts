'use client';

import { createContext, useContext } from 'react';

export type CookieContextType = {
  isLoggedIn: boolean;
  userId?: string;
  token?: string;
  id?: string;
};

export const CookieContext = createContext<CookieContextType | undefined>(
  undefined,
);

export function useCookie() {
  const context = useContext(CookieContext);
  if (!context) {
    throw new Error('useCookie must be used within a CookieProvider');
  }
  return context;
}
