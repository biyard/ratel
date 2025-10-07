import { Ed25519KeyIdentity } from '@dfinity/identity';
import { logger } from '../logger';
import { decode_base64, encode_base64 } from '../base64';

export function createEd25519KeyPair(): Ed25519KeyIdentity {
  const seed = new Uint8Array(32);
  crypto.getRandomValues(seed);

  const ed25519 = Ed25519KeyIdentity.generate(seed);

  logger.debug('ed25519 principal', ed25519.getPrincipal());

  return ed25519;
}

export function encodeEd25519PrivateKeyToPkcs8Base64(
  identity: Ed25519KeyIdentity,
): string {
  const secret = identity.getKeyPair().secretKey;

  const encoded = encode_base64(new Uint8Array(secret));

  logger.debug('Encoded private key:', encoded);

  return `v2:${encoded}`;
}

export function restoreEd25519KeyPair(backup_str: string): Ed25519KeyIdentity {
  if (backup_str.startsWith('v2:')) {
    return restoreV2Ed25519KeyPair(backup_str);
  } else {
    return restoreLegacyEd25519KeyPair(backup_str);
  }
}

export function restoreV2Ed25519KeyPair(
  backup_str: string,
): Ed25519KeyIdentity {
  if (!backup_str.startsWith('v2:')) {
    throw new Error('Invalid backup string format. Expected "v2:" prefix.');
  }

  const base64 = backup_str.slice(3);
  const secret = decode_base64(base64);

  logger.debug('secret key bytes after base64 decoding: ', secret);

  const identity = Ed25519KeyIdentity.fromSecretKey(
    secret.buffer as ArrayBuffer,
  );

  logger.debug('Principal:', identity.getPrincipal());

  return identity;
}

export function restoreLegacyEd25519KeyPair(
  backup_str: string,
): Ed25519KeyIdentity {
  const pkcs8 = decode_base64(backup_str);

  logger.debug('private key bytes after base64 decoding: ', pkcs8);

  const privateKey = pkcs8.buffer.slice(16, 48) as ArrayBuffer;
  const publicKey = pkcs8.buffer.slice(51, 83) as ArrayBuffer;

  logger.debug('Private Key:', privateKey);
  logger.debug('Public Key:', publicKey);

  const identity = Ed25519KeyIdentity.fromKeyPair(publicKey, privateKey);

  logger.debug('Principal:', identity.getPrincipal());

  return identity;
}
