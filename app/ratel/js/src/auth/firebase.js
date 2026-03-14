import { initializeApp } from "firebase/app";
import {
  GoogleAuthProvider,
  getAuth,
  signInWithPopup,
  signOut,
} from "firebase/auth";

let app;
let auth;
let provider;
let initialized = false;

export function init_firebase(conf) {
  try {
    app = initializeApp(conf);
    auth = getAuth(app);
    provider = new GoogleAuthProvider();

    initialized = true;
  } catch (e) {
    console.error("Firebase initialization failed:", e);
  }
}

export async function signIn() {
  if (!initialized) {
    console.error("Firebase initialization failed:");
  }

  try {
    const result = await signInWithPopup(auth, provider);
    const user = result.user;
    const credential = GoogleAuthProvider.credentialFromResult(result);
    const accessToken = credential?.accessToken ?? "";
    const idToken = await user.getIdToken();

    return {
      access_token: accessToken,
      id_token: idToken,
      email: user.email,
      display_name: user.displayName,
      photo_url: user.photoURL,
    };
  } catch (e) {
    console.error(e);
  }
}
