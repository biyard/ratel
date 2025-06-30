'use client';
/* eslint-disable @typescript-eslint/no-explicit-any */
import { config } from '@/config';
import { logger } from '../logger';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { encode_base64 } from '../base64';

export async function send(
  keyPair: Ed25519KeyIdentity,
  path: string,
  apiBaseUrl: string = config.api_url,
): Promise<any | undefined> {
  const pk = keyPair.getPublicKey().rawKey;
  const publicKey = encode_base64(new Uint8Array(pk));
  const timestamp = Math.floor(Date.now() / 1000);
  const msg = `${config.sign_domain}-${timestamp}`;
  logger.debug('Signing message:', msg, 'with public key:', publicKey);
  const encoder = new TextEncoder();
  const msg_bytes = encoder.encode(msg).buffer as ArrayBuffer;
  const sig = await keyPair.sign(msg_bytes);

  logger.debug('Signature:', sig, 'Public Key:', pk);

  const s = encode_base64(new Uint8Array(sig.slice()));

  const token_type = 'UserSig';
  const token = `${timestamp}:eddsa:${publicKey}:${s}`;

  logger.debug('sending request', path);
  // const apiBaseUrl = config.api_url;
  logger.debug('send', token, apiBaseUrl);

  logger.debug(
    'Sending request to:',
    `${apiBaseUrl}${path}`,
    'with token:',
    token_type,
    token,
  );

  const req: any = {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
  };

  if (token !== '') {
    req.headers['Authorization'] = `${token_type} ${token}`;
  }

  logger.debug(`${apiBaseUrl}${path}`, req);

  const response = await fetch(`${apiBaseUrl}${path}`, req);

  if (!response.ok) {
    return undefined;
  }

  return response.json();
}
