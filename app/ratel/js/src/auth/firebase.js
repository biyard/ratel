let firebaseConf = null;
let app;
let auth;
let provider;
let initialized = false;
let initPromise = null;

async function ensureFirebase() {
  if (initialized) return;
  if (initPromise) {
    await initPromise;
    return;
  }
  if (!firebaseConf) {
    throw new Error("Firebase config not set. Call init_firebase first.");
  }

  initPromise = (async () => {
    const { initializeApp } = await import("firebase/app");
    const { GoogleAuthProvider, getAuth } = await import("firebase/auth");
    app = initializeApp(firebaseConf);
    auth = getAuth(app);
    provider = new GoogleAuthProvider();
    initialized = true;
  })();

  await initPromise;
}

// Synchronous — just saves config. Firebase SDK loaded lazily on first signIn.
export function init_firebase(conf) {
  firebaseConf = conf;
}

export async function signIn() {
  try {
    await ensureFirebase();
  } catch (e) {
    console.error("Firebase initialization failed:", e);
    return;
  }

  try {
    const { signInWithPopup, GoogleAuthProvider } = await import(
      "firebase/auth"
    );
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
