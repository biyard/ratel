/* eslint-disable @typescript-eslint/no-explicit-any */

'use client';

import { config } from '@/config';
import { useCookie } from '../contexts/cookie-context';
import { useAuth } from '../contexts/auth-context';
import { logger } from '../logger';
import { encode_base64 } from '../base64';
import { Ed25519KeyIdentity } from '@dfinity/identity';

export interface ApiCallFns {
  get: <T = any>(path: string) => Promise<T>;
  post: <T = any>(path: string, body?: any) => Promise<T>;
}

export function useApiCall(): ApiCallFns {
  const auth = useAuth();
  const cookie = useCookie();

  return {
    get: async <T = any>(path: string): Promise<T> => {
      const apiBaseUrl: string = config.api_url;
      let token = cookie?.token;
      let token_type = 'Bearer';

      if (!token && auth.ed25519KeyPair) {
        const keyPair = auth.ed25519KeyPair;
        const pk = keyPair.getPublicKey().rawKey;
        const publicKey = encode_base64(new Uint8Array(pk));
        const timestamp = Math.floor(Date.now() / 1000);
        const msg = `${config.sign_domain}-${timestamp}`;
        logger.debug('Signing message:', msg, 'with public key:', publicKey);
        const encoder = new TextEncoder();
        const msg_bytes = encoder.encode(msg).buffer as ArrayBuffer;
        const sig = await keyPair.sign(msg_bytes);

        logger.debug('Signature:', sig, 'Public Key:', pk);

        logger.debug(
          'verified: ',
          Ed25519KeyIdentity.verify(sig, msg_bytes, pk),
        );
        const s = encode_base64(new Uint8Array(sig.slice()));

        token_type = 'UserSig';
        token = `${timestamp}:eddsa:${publicKey}:${s}`;
      }

      const headers: any = { 'Content-Type': 'application/json' };
      if (token) headers['Authorization'] = `${token_type} ${token}`;

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'GET',
        headers,
        credentials: 'include',
      });

      if (!response.ok) {
        return null as unknown as T;
      }

      return response.json() as Promise<T>;
    },
    post: async <T = any>(path: string, body?: any): Promise<T> => {
      const apiBaseUrl: string = config.api_url;
      let token = cookie?.token;
      let token_type = 'Bearer';

      if (!token && auth.ed25519KeyPair) {
        const keyPair = auth.ed25519KeyPair;
        const pk = keyPair.getPublicKey().rawKey;
        const publicKey = encode_base64(new Uint8Array(pk));
        const timestamp = Math.floor(Date.now() / 1000);
        const msg = `${config.sign_domain}-${timestamp}`;
        logger.debug('Signing message:', msg, 'with public key:', publicKey);
        const encoder = new TextEncoder();
        const msg_bytes = encoder.encode(msg).buffer as ArrayBuffer;
        const sig = await keyPair.sign(msg_bytes);

        logger.debug('Signature:', sig, 'Public Key:', pk);

        logger.debug(
          'verified: ',
          Ed25519KeyIdentity.verify(sig, msg_bytes, pk),
        );
        const s = encode_base64(new Uint8Array(sig.slice()));

        token_type = 'UserSig';
        token = `${timestamp}:eddsa:${publicKey}:${s}`;
      }

      const headers: any = { 'Content-Type': 'application/json' };
      if (token) headers['Authorization'] = `${token_type} ${token}`;

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'POST',
        headers,
        credentials: 'include',
        body: body ? JSON.stringify(body) : undefined,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({
          message: 'Failed to fetch and parse error',
        }));
        logger.error('Failed to fetch and parse error ', errorData?.message);
        return null as unknown as T;
      }

      return response.json() as Promise<T>;
    },
  };
}
