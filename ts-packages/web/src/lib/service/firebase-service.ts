/* eslint-disable @typescript-eslint/no-explicit-any */
// services/firebaseService.ts
import { initializeApp } from 'firebase/app';
import {
  getAuth,
  signInWithPopup,
  GoogleAuthProvider,
  signOut,
  onAuthStateChanged,
  type User,
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

// Firebase configuration validation
const validateFirebaseConfig = (config: any): boolean => {
  const requiredFields = [
    'apiKey',
    'authDomain',
    'projectId',
    'storageBucket',
    'messagingSenderId',
    'appId',
  ];

  for (const field of requiredFields) {
    const value = config[field];
    if (!value || value === '' || value === 'undefined' || value === 'null') {
      logger.warn(`Firebase config missing or invalid: ${field}`);
      return false;
    }
  }

  // Validate API key format (should start with AIza for Google APIs)
  if (!config.apiKey.startsWith('AIza')) {
    logger.warn('Firebase API key appears to be invalid format');
    return false;
  }

  // Validate project ID format (should not contain invalid characters)
  if (!/^[a-z0-9-]+$/.test(config.projectId)) {
    logger.warn('Firebase project ID contains invalid characters');
    return false;
  }

  return true;
};

const firebaseConfig = {
  apiKey: config.firebase_api_key,
  authDomain: config.firebase_auth_domain,
  projectId: config.firebase_project_id,
  storageBucket: config.firebase_storage_bucket,
  messagingSenderId: config.firebase_messaging_sender_id,
  appId: config.firebase_app_id,
  measurementId: config.firebase_measurement_id,
};

// Check if Firebase should be initialized
const isFirebaseConfigValid = validateFirebaseConfig(firebaseConfig);
if (!isFirebaseConfigValid) {
  logger.warn('Firebase configuration is invalid or incomplete.');
}

export interface AuthUserInfo {
  idToken: string;
  accessToken: string;
  email: string | null;
  displayName: string | null;
  photoURL: string | null;
}

// Conditional Firebase initialization
let app: any = null;
let auth: any = null;
let provider: GoogleAuthProvider | null = null;

if (isFirebaseConfigValid) {
  try {
    app = initializeApp(firebaseConfig);
    auth = getAuth(app);
    provider = new GoogleAuthProvider();
    logger.debug('🔥 Firebase initialized successfully');
  } catch (error) {
    logger.error('🔥 Failed to initialize Firebase:', error);
    app = null;
    auth = null;
    provider = null;
  }
} else {
  logger.warn('🔥 Skipping Firebase initialization');
}

export const EventType = {
  Login: 1,
  SignUp: 2,
} as const;

export type EventType = (typeof EventType)[keyof typeof EventType];
export type GoogleLoginInfo = {
  idToken: string;
  accessToken: string;

  email: string | null;
  displayName: string | null;
  photoURL: string | null;
};

// Helper function to check if Firebase is available
export const isFirebaseAvailable = (): boolean => {
  return auth !== null && provider !== null;
};

export const loginWithGoogle = async (): Promise<GoogleLoginInfo> => {
  // Check if Firebase is properly initialized
  if (!isFirebaseAvailable()) {
    throw new Error(
      'Firebase is not available. Please check your Firebase configuration.',
    );
  }

  if (!provider || !auth) {
    throw new Error('Firebase authentication is not properly initialized.');
  }

  try {
    const result = await signInWithPopup(auth, provider);
    const user = result.user;
    logger.debug('Google user info:', user);
    const accessToken =
      GoogleAuthProvider.credentialFromResult(result)?.accessToken ?? '';
    const idToken = await user.getIdToken();

    logger.debug('id Token: ', idToken, ', accessToken:', accessToken);

    return {
      idToken,
      accessToken,
      email: user.email,
      photoURL: user.photoURL,
      displayName: user.displayName,
    };
  } catch (error) {
    logger.error('🔥 Google login failed:', error);
    throw new Error(
      `Google login failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
    );
  }
};

export const logout = async () => {
  if (!auth) {
    logger.warn('🔥 Cannot logout: Firebase auth not initialized');
    return;
  }

  try {
    await signOut(auth);
  } catch (error) {
    logger.error('🔥 Logout failed:', error);
    throw new Error(
      `Logout failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
    );
  }
};

export const onUserChanged = (
  cb: (user: User | null) => void,
): (() => void) => {
  if (!auth) {
    logger.warn('🔥 Firebase auth not initialized');
    // Return a no-op function
    return () => {};
  }

  return onAuthStateChanged(auth, cb);
};

export const getAuthInstance = () => {
  if (!auth) {
    logger.warn('🔥 Firebase auth not initialized');
  }
  return auth;
};

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
