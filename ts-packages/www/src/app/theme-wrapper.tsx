'use client';

import * as React from 'react';
import { ThemeType } from '@/lib/api/models/user';
import { useUserInfo } from '@/hooks/use-user-info';

// Placeholder for ThemeProvider - next-themes is not available in this project
const ThemeProvider = ({ children }: { children: React.ReactNode; attribute?: string; defaultTheme?: string; enableSystem?: boolean; forcedTheme?: string; disableTransitionOnChange?: boolean }) => <>{children}</>;

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data } = useUserInfo();

  let theme = 'system';
  if (data?.theme === ThemeType.Dark) {
    theme = 'dark';
  } else if (data?.theme === ThemeType.Light) {
    theme = 'light';
  }

  return (
    <ThemeProvider
      attribute="data-theme"
      defaultTheme="system"
      enableSystem
      forcedTheme={theme}
      disableTransitionOnChange
    >
      {children}
    </ThemeProvider>
  );
}
