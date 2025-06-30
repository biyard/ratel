'use client';

import { createEd25519KeyPair } from '@/lib/wallet/ed25519';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import React, { createContext, useContext, useEffect, useState } from 'react';

type KeyPair = {
  ed25519: Ed25519KeyIdentity;
} | null;

const KeyPairContext = createContext<KeyPair>(null);

export function KeyPairProvider({ children }: { children: React.ReactNode }) {
  const [keyPair, setKeyPair] = useState<KeyPair>(null);

  useEffect(() => {
    setKeyPair({ ed25519: createEd25519KeyPair() });
  }, []);

  return (
    <KeyPairContext.Provider value={keyPair}>
      {children}
    </KeyPairContext.Provider>
  );
}

export function useKeyPair(): KeyPair {
  return useContext(KeyPairContext);
}
