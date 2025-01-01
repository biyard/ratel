import { initializeApp as ia } from "https://www.gstatic.com/firebasejs/11.1.0/firebase-app.js";
import {
  GoogleAuthProvider,
  getAuth as a,
  signInWithPopup as sp,
  signOut as so,
} from "https://www.gstatic.com/firebasejs/11.1.0/firebase-auth.js";

export function initializeApp(
  apiKey,
  authDomain,
  projectId,
  storageBucket,
  messagingSenderId,
  appId,
  measurementId,
) {
  ia({
    apiKey,
    authDomain,
    projectId,
    storageBucket,
    messagingSenderId,
    appId,
    measurementId,
  });
}

export async function signInWithPopup(scopes) {
  const auth = a();
  const provider = new GoogleAuthProvider();
  for (const scope of scopes) {
    provider.addScope(scope);
  }

  try {
    const res = await sp(auth, provider);
    const credential = GoogleAuthProvider.credentialFromResult(res);
    const access_token = credential?.accessToken;
    const id_token = await auth.currentUser.getIdToken(true);

    return JSON.stringify({
      id_token,
      access_token,
      display_name: res.user.displayName,
      email: res.user.email,
      photo_url: res.user.photoURL,
    });
  } catch (e) {
    return JSON.stringify({
      id_token: "",
      access_token: "",
      display_name: "",
      email: "",
      photo_url: "",
    });
  }
}

export function signOut(auth) {
  if (auth.currentUser) {
    signOut(auth);
  }
}
