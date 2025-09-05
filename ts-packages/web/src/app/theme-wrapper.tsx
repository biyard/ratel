'use client';

import { ThemeProvider } from 'next-themes';

export default function ThemeWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ThemeProvider
      attribute="data-theme"
      defaultTheme="system"
      enableSystem
      storageKey="theme"
    >
      {children}
    </ThemeProvider>
  );
}
