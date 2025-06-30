'use client';

import { ReactNode } from 'react';
import {
  CookieContext,
  CookieContextType,
} from '@/lib/contexts/cookie-context';

export default function CookieProviderClient({
  children,
  value,
}: {
  children: ReactNode;
  value: CookieContextType;
}) {
  return (
    <CookieContext.Provider value={value}>{children}</CookieContext.Provider>
  );
}
