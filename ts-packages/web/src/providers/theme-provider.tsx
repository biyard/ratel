'use client';

import { useTheme } from '@/hooks/use-theme';

/**
 * ThemeProvider ensures the theme is applied globally across the entire app.
 *
 * This component must be placed at the root level to ensure:
 * 1. Theme persists across all pages (not just settings)
 * 2. Theme is re-applied when navigating between routes
 * 3. Theme hook is always active to listen for changes
 *
 * Note: The initial theme is applied via inline script in index.html
 * to prevent flash of unstyled content (FOUC) on page load.
 */
export function ThemeProvider({ children }: { children: React.ReactNode }) {
  // Calling useTheme at the root ensures theme state is maintained
  // and re-applied across all navigation and page loads
  useTheme();

  return <>{children}</>;
}
