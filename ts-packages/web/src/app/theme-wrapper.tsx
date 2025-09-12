'use client';

import { ThemeProvider } from 'next-themes';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { ThemeType } from '@/lib/api/models/user';

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data } = useSuspenseUserInfo();

  const apiTheme =
    data?.theme === ThemeType.Dark
      ? 'dark'
      : data?.theme === ThemeType.Light
        ? 'light'
        : data?.theme === ThemeType.SystemDefault
          ? 'system'
          : undefined;

  const forced = apiTheme ?? 'system';

  return (
    <ThemeProvider
      attribute="data-theme"
      forcedTheme={forced}
      enableSystem={false}
    >
      {children}
    </ThemeProvider>
  );
}
