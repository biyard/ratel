'use client';

import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';

export type ThemeSetting = 'dark' | 'light' | 'system';
export type ResolvedTheme = 'dark' | 'light';

const STORAGE_KEY = 'ratel.theme';

type ThemeContextType = {
  themeSetting: ThemeSetting; // user preference
  resolvedTheme: ResolvedTheme; // effective theme after resolving 'system'
  setTheme: (theme: ThemeSetting) => void;
  toggleTheme: () => void; // toggles between dark/light (keeps out of 'system')
};

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

function getSystemTheme(): ResolvedTheme {
  if (typeof window === 'undefined') return 'dark';
  return window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light';
}

function applyThemeToDom(theme: ResolvedTheme) {
  // Our CSS defaults to dark, light is an override via data-theme="light"
  const el = document.documentElement;
  if (theme === 'light') {
    el.setAttribute('data-theme', 'light');
  } else {
    el.removeAttribute('data-theme'); // dark is default
  }
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const isMounted = useRef(false);
  // Initial state: try to read from localStorage if available; otherwise system
  const [themeSetting, setThemeSetting] = useState<ThemeSetting>(() => {
    if (typeof window === 'undefined') return 'dark';
    try {
      const saved = window.localStorage.getItem(
        STORAGE_KEY,
      ) as ThemeSetting | null;
      return saved ?? 'system';
    } catch {
      return 'system';
    }
  });

  const resolvedTheme: ResolvedTheme = useMemo(() => {
    return themeSetting === 'system' ? getSystemTheme() : themeSetting;
  }, [themeSetting]);

  // Apply to DOM on changes
  useEffect(() => {
    if (!isMounted.current) {
      isMounted.current = true;
    }
    applyThemeToDom(resolvedTheme);
  }, [resolvedTheme]);

  // Persist preference (only when not system or explicitly save system)
  useEffect(() => {
    try {
      window.localStorage.setItem(STORAGE_KEY, themeSetting);
    } catch {}
  }, [themeSetting]);

  // Listen to system theme changes when in 'system'
  useEffect(() => {
    if (themeSetting !== 'system') return;
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    const modernHandler = (e: MediaQueryListEvent) => {
      applyThemeToDom(e.matches ? 'dark' : 'light');
    };
    const legacyHandler: (
      this: MediaQueryList,
      ev: MediaQueryListEvent,
    ) => void = function (this: MediaQueryList, e: MediaQueryListEvent) {
      applyThemeToDom(e.matches ? 'dark' : 'light');
    };
    if (mq.addEventListener) mq.addEventListener('change', modernHandler);
    else mq.addListener(legacyHandler);
    return () => {
      if (mq.removeEventListener)
        mq.removeEventListener('change', modernHandler);
      else mq.removeListener(legacyHandler);
    };
  }, [themeSetting]);

  // Cross-tab sync
  useEffect(() => {
    const onStorage = (e: StorageEvent) => {
      if (e.key !== STORAGE_KEY) return;
      const next = (e.newValue as ThemeSetting | null) ?? 'system';
      setThemeSetting(next);
    };
    window.addEventListener('storage', onStorage);
    return () => window.removeEventListener('storage', onStorage);
  }, []);

  const setTheme = useCallback((theme: ThemeSetting) => {
    setThemeSetting(theme);
  }, []);

  const toggleTheme = useCallback(() => {
    setThemeSetting((prev) => (prev === 'light' ? 'dark' : 'light'));
  }, []);

  const value = useMemo(
    () => ({ themeSetting, resolvedTheme, setTheme, toggleTheme }),
    [themeSetting, resolvedTheme, setTheme, toggleTheme],
  );

  return (
    <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
  );
}

export function useTheme() {
  const ctx = useContext(ThemeContext);
  if (!ctx) throw new Error('useTheme must be used within ThemeProvider');
  return ctx;
}
