'use client';

import { useTheme } from './_providers/ThemeProvider';

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const { theme } = useTheme();

  return (
    <div className={theme === 'light' ? 'bg-light-bg' : 'bg-bg'}>
      {children}
    </div>
  );
}
