'use client';

import { createContext, useContext } from 'react';
import type { User } from 'firebase/auth';
import type { AuthUserInfo } from '../service/firebase-service';

interface AuthContextType {
  telegramRaw?: string;
  user?: User;
  authUser?: AuthUserInfo;
  login: () => Promise<AuthUserInfo>;
  logout: () => Promise<void>;
  setTelegramRaw: (raw: string) => void;
}

const dummyAuthUserInfo: AuthUserInfo = {
  email: '',
  displayName: '',
  photoURL: '',
  idToken: '',
  accessToken: '',
};

export const AuthContext = createContext<AuthContextType>({
  login: async () => dummyAuthUserInfo,
  logout: async () => {},
  setTelegramRaw: () => {},
});

export const useAuth = () => useContext(AuthContext);
