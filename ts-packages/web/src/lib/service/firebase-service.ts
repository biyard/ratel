// services/firebaseService.ts
import { initializeApp } from 'firebase/app';
import {
  getAuth,
  signInWithPopup,
  GoogleAuthProvider,
  signOut,
  onAuthStateChanged,
  User,
} from 'firebase/auth';
import { getFile, listFiles, uploadFile } from '../api/drive';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { toHex } from '@dfinity/agent';
import { config } from '@/config';
import { logger } from '../logger';
import {
  encodeEd25519PrivateKeyToPkcs8Base64,
  restoreEd25519KeyPair,
} from '../wallet/ed25519';

const firebaseConfig = {
  apiKey: config.firebase_api_key,
  authDomain: config.firebase_auth_domain,
  projectId: config.firebase_project_id,
  storageBucket: config.firebase_storage_bucket,
  messagingSenderId: config.firebase_messaging_sender_id,
  appId: config.firebase_app_id,
  measurementId: config.firebase_measurement_id,
};

export interface AuthUserInfo {
  principal: string | null;
  event: EventType | null;
  contents: string | null;
  email: string | null;
  displayName: string | null;
  photoURL: string | null;
  keyPair: Ed25519KeyIdentity | null;
}

const app = initializeApp(firebaseConfig);
const auth = getAuth(app);
const provider = new GoogleAuthProvider();

export enum EventType {
  Login = 1,
  SignUp = 2,
}
export type GoogleLoginInfo = {
  eventType: EventType;
  keyPair: Ed25519KeyIdentity;
  contents: string;

  email: string | null;
  displayName: string | null;
  photoURL: string | null;
  principal: string | null;
};

export const loginWithGoogle = async (
  anonKeyPair: Ed25519KeyIdentity,
): Promise<GoogleLoginInfo> => {
  provider.addScope('https://www.googleapis.com/auth/drive.appdata');
  const result = await signInWithPopup(auth, provider);
  const user = result.user;
  const accessToken =
    GoogleAuthProvider.credentialFromResult(result)?.accessToken ?? '';
  const idToken = await user.getIdToken();

  logger.debug('id Token: ', idToken, ', accessToken:', accessToken);

  const files = await listFiles(config.env, accessToken);

  logger.debug('file data: ', files);

  let eventType = EventType.Login;
  let keyPair;
  let contents;

  if (files.length > 0) {
    const file = files[0];
    try {
      contents = await getFile(accessToken, file.id);
      keyPair = restoreEd25519KeyPair(contents);
    } catch (e) {
      logger.error('Failed to get file content', e);
      throw new Error('failed to get file');
    }
  } else {
    logger.debug('key pair: ', anonKeyPair);

    contents = encodeEd25519PrivateKeyToPkcs8Base64(anonKeyPair);

    try {
      const res = await uploadFile(accessToken, contents);

      logger.debug('upload data', res);
      eventType = EventType.SignUp;
      keyPair = anonKeyPair;
    } catch (e) {
      logger.error('Failed to upload file content', e);
      throw new Error('failed to upload file');
    }
  }

  logger.debug(
    'principal(google signed in): ',
    keyPair.getPrincipal().toText(),
  );

  return {
    keyPair,
    eventType,
    contents,
    email: user.email,
    photoURL: user.photoURL,
    displayName: user.displayName,
    principal: keyPair.getPrincipal().toText(),
  };
};

export const logout = async () => await signOut(auth);

export const onUserChanged = (
  cb: (user: User | null) => void,
): (() => void) => {
  return onAuthStateChanged(auth, cb);
};

export const getAuthInstance = () => auth;

function base64ToUint8Array(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}
export const trySetupFromPrivateKey = async (privateKeyBase64: string) => {
  try {
    const pkcs8 = base64ToUint8Array(privateKeyBase64);

    logger.debug('private key bytes after base64 decoding: ', pkcs8);

    const privateKey = pkcs8.buffer.slice(19, 51) as ArrayBuffer;
    const publicKey = pkcs8.buffer.slice(51, 83) as ArrayBuffer;

    logger.debug('Private Key:', privateKey);
    logger.debug('Public Key:', publicKey);

    const identity = Ed25519KeyIdentity.fromKeyPair(publicKey, privateKey);

    const principal = identity.getPrincipal().toText();
    const publicKeyHex = toHex(identity.getPublicKey().toDer());

    logger.debug('Principal:', principal);
    logger.debug('Public Key:', publicKeyHex);

    return {
      privateKeyBase64,
      principal,
      publicKeyHex,
      identity,
    };
  } catch (err) {
    logger.error('Failed to setup identity from private key:', err);
    return null;
  }
};
