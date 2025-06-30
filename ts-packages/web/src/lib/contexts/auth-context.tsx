'use client';

import { createContext, useContext } from 'react';
import { User } from 'firebase/auth';
import { AuthUserInfo } from '../service/firebase-service';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { NoEd25519KeyPair } from '@/errors';
import { HDNodeWallet } from 'ethers';

interface AuthContextType {
  ed25519KeyPair: Ed25519KeyIdentity | null;
  evmWallet?: HDNodeWallet;
  user?: User;
  authUser?: AuthUserInfo;
  login: (keyPair: Ed25519KeyIdentity) => Promise<AuthUserInfo>;
  logout: () => Promise<void>;
}

const dummyAuthUserInfo: AuthUserInfo = {
  principal: '',
  contents: '',
  email: '',
  displayName: '',
  photoURL: '',
  event: null,
  keyPair: null,
};

export const AuthContext = createContext<AuthContextType>({
  ed25519KeyPair: null,
  login: async () => dummyAuthUserInfo,
  logout: async () => {},
});

export const useAuth = () => useContext(AuthContext);

export function useEd25519KeyPair(): Ed25519KeyIdentity {
  const { ed25519KeyPair } = useAuth();

  if (!ed25519KeyPair) throw NoEd25519KeyPair;

  return ed25519KeyPair!;
}
