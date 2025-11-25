'use client';

import { useEffect, useState } from 'react';
import { User } from 'firebase/auth';
import {
  AuthUserInfo,
  loginWithGoogle as loginWithGoogle,
  logout,
  onUserChanged,
} from '@/lib/service/firebase-service';
import { AuthContext } from '@/lib/contexts/auth-context';
import { logger } from '@/lib/logger';
import { useQueryClient } from '@tanstack/react-query';
import { config } from '@/config';
import { removeUserInfo } from '@/hooks/use-user-info';
import { trackLogin } from '@/lib/service/analytics-service';

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [user, setUser] = useState<User | undefined>(undefined);
  const [authUser, setAuthUser] = useState<AuthUserInfo | undefined>(undefined);
  const [telegramRaw, setRaw] = useState<string | undefined>(undefined);

  const queryClient = useQueryClient();
  useEffect(() => {
    const unsubscribe = onUserChanged((user) => {
      setUser(user || undefined);
    });

    return () => unsubscribe();
  }, []);

  const login = async (): Promise<AuthUserInfo> => {
    logger.debug('login');
    const info = await loginWithGoogle();
    const authInfo: AuthUserInfo = {
      email: info.email,
      displayName: '',
      photoURL: info.photoURL,
      accessToken: info.accessToken,
      idToken: info.idToken,
    };

    setAuthUser(authInfo);

    // Track login event
    trackLogin('google');

    return authInfo;
  };

  const logoutUser = async () => {
    await logout();
    const url = config.api_url;
    await fetch(`${url}/v3/auth/logout`, {
      method: 'POST',
      credentials: 'include',
    });
    setUser(undefined);
    setAuthUser(undefined);
    removeUserInfo(queryClient);

    // Redirect to home page after logout
    window.location.href = '/';
  };

  const setTelegramRaw = (raw: string) => {
    setRaw(raw);
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        authUser,
        login,
        logout: logoutUser,
        telegramRaw,
        setTelegramRaw,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
