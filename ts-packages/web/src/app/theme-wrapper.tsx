'use client';

import * as React from 'react';
import { useTheme } from '@/hooks/use-theme';

// Placeholder for ThemeProvider - next-themes is not available in this project
const ThemeProvider = ({
  children,
}: {
  children: React.ReactNode;
  attribute?: string;
  defaultTheme?: string;
  enableSystem?: boolean;
  forcedTheme?: string;
  disableTransitionOnChange?: boolean;
}) => <>{children}</>;

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const { theme } = useTheme();

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
