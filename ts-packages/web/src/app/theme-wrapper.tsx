'use client';

import { ThemeProvider } from 'next-themes';
import { ThemeType } from '@/lib/api/models/user';
import { useUserInfo } from '@/hooks/use-user-info';

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
