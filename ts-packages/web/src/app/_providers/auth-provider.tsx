'use client';

import React, { useEffect, useState } from 'react';
import { User } from 'firebase/auth';
import {
  AuthUserInfo,
  loginWithGoogle as loginWithGoogle,
  logout,
  onUserChanged,
} from '@/lib/service/firebase-service';
import { SK_ANONYMOUS_IDENTITY_KEY, SK_IDENTITY_KEY } from '@/constants';
import { AuthContext } from '@/lib/contexts/auth-context';
import { logger } from '@/lib/logger';
import {
  createEd25519KeyPair,
  encodeEd25519PrivateKeyToPkcs8Base64,
  restoreEd25519KeyPair,
} from '@/lib/wallet/ed25519';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { HDNodeWallet } from 'ethers';
import { removeUserInfo } from '@/lib/api/hooks/users';
import { useQueryClient } from '@tanstack/react-query';

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [user, setUser] = useState<User | undefined>(undefined);
  const [authUser, setAuthUser] = useState<AuthUserInfo | undefined>(undefined);
  const [ed25519KeyPair, setEd25519KeyPair] =
    useState<Ed25519KeyIdentity | null>(null);
  const [evmWallet, setEvmWallet] = useState<HDNodeWallet | undefined>(
    undefined,
  );

  const queryClient = useQueryClient();
  useEffect(() => {
    const stored = localStorage.getItem(SK_IDENTITY_KEY);
    let identity = null;

    try {
      if (stored) {
        // for our users
        identity = restoreEd25519KeyPair(stored);
        logger.debug('Restored principal:', identity.getPrincipal().toText());

        setEd25519KeyPair(identity);
        localStorage.setItem(SK_ANONYMOUS_IDENTITY_KEY, stored);
      } else {
        // for guests
        const stored = localStorage.getItem(SK_ANONYMOUS_IDENTITY_KEY);

        if (stored) {
          // previousely visited
          identity = restoreEd25519KeyPair(stored);
          setEd25519KeyPair(identity);

          logger.debug(
            'Restored anonymous principal:',
            identity.getPrincipal().toText(),
          );
        } else {
          // first visited
          identity = createEd25519KeyPair();

          logger.debug(
            'Created new principal:',
            identity.getPrincipal().toText(),
          );

          setEd25519KeyPair(identity);
          const encoded_identity =
            encodeEd25519PrivateKeyToPkcs8Base64(identity);

          localStorage.setItem(SK_ANONYMOUS_IDENTITY_KEY, encoded_identity);
        }
      }
    } catch (error) {
      logger.error('Failed to restore identity:', error);
      localStorage.removeItem(SK_IDENTITY_KEY);
      localStorage.removeItem(SK_ANONYMOUS_IDENTITY_KEY);

      identity = createEd25519KeyPair();

      logger.debug('Created new principal:', identity.getPrincipal().toText());

      setEd25519KeyPair(identity);
      const encoded_identity = encodeEd25519PrivateKeyToPkcs8Base64(identity);

      localStorage.setItem(SK_ANONYMOUS_IDENTITY_KEY, encoded_identity);
    }

    const secret = identity!.getKeyPair().secretKey;
    logger.debug('secret', secret);
    const wallet = HDNodeWallet.fromSeed(new Uint8Array(secret as ArrayBuffer));
    setEvmWallet(wallet);
    logger.debug('EVM wallet created:', wallet.address);

    const unsubscribe = onUserChanged((user) => {
      setUser(user || undefined);
    });

    return () => unsubscribe();
  }, []);

  const login = async (keyPair: Ed25519KeyIdentity): Promise<AuthUserInfo> => {
    logger.debug('login');
    const info = await loginWithGoogle(keyPair);
    const authInfo: AuthUserInfo = {
      principal: info.principal,
      event: info.eventType,
      contents: info.contents,
      email: info.email,
      displayName: '',
      photoURL: info.photoURL,
      keyPair: info.keyPair,
    };

    localStorage.setItem(SK_IDENTITY_KEY, info.contents);
    localStorage.setItem(SK_ANONYMOUS_IDENTITY_KEY, info.contents);

    setAuthUser(authInfo);
    setEd25519KeyPair(info.keyPair);

    const secret = info.keyPair!.getKeyPair().secretKey;
    logger.debug('secret', secret);
    const wallet = HDNodeWallet.fromSeed(new Uint8Array(secret as ArrayBuffer));
    setEvmWallet(wallet);
    logger.debug('EVM wallet created:', wallet.address);
    return authInfo;
  };

  const logoutUser = async () => {
    await logout();
    await fetch('/api/logout', { method: 'POST' });
    setUser(undefined);
    setAuthUser(undefined);
    localStorage.removeItem(SK_IDENTITY_KEY);
    localStorage.removeItem(SK_ANONYMOUS_IDENTITY_KEY);

    const identity = createEd25519KeyPair();

    logger.debug('Created new principal:', identity.getPrincipal().toText());

    setEd25519KeyPair(identity);

    removeUserInfo(queryClient);

    const secret = identity!.getKeyPair().secretKey;
    logger.debug('secret', secret);
    const wallet = HDNodeWallet.fromSeed(new Uint8Array(secret as ArrayBuffer));
    setEvmWallet(wallet);
    logger.debug('EVM wallet created:', wallet.address);

    const encoded_identity = encodeEd25519PrivateKeyToPkcs8Base64(identity);

    localStorage.setItem(SK_ANONYMOUS_IDENTITY_KEY, encoded_identity);
  };

  return (
    <AuthContext.Provider
      value={{
        ed25519KeyPair,
        user,
        authUser,
        login,
        logout: logoutUser,
        evmWallet,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};
