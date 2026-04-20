import { initializeApp } from "https://www.gstatic.com/firebasejs/12.12.0/firebase-app.js";

import {
  GoogleAuthProvider,
  getAuth,
  signInWithPopup,
} from "https://www.gstatic.com/firebasejs/12.12.0/firebase-auth.js";

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

// Detects common in-app browsers that block Google OAuth. Returns one of:
//   "kakaotalk" | "instagram" | "facebook" | "line" | "naver" |
//   "daum" | "other-inapp" | ""
// Empty string means a regular browser where OAuth works.
export function detectInAppBrowser() {
  if (typeof navigator === "undefined") return "";
  const ua = (navigator.userAgent || "").toLowerCase();

  if (ua.includes("kakaotalk")) return "kakaotalk";
  // Facebook in-app: FBAN / FBAV / FB_IAB / FBIOS
  if (/\bfbav\b|\bfban\b|\bfb_iab\b|\bfbios\b/.test(ua)) return "facebook";
  if (ua.includes("instagram")) return "instagram";
  if (/\bline\//.test(ua)) return "line";
  if (ua.includes("naver(inapp") || ua.includes("naver(inapp;")) return "naver";
  if (ua.includes("daumapps")) return "daum";

  // Heuristic: Android WebView without Chrome UI usually means some other
  // in-app browser. iOS detection is harder — we err on the side of not
  // blocking unknown browsers.
  const isAndroid = ua.includes("android");
  const isWv = ua.includes("; wv)");
  if (isAndroid && isWv) return "other-inapp";

  return "";
}

// Attempts to escape a KakaoTalk in-app webview to the user's default
// browser. MUST be called from a user gesture (click handler) — modern
// WebViews silently ignore custom-scheme redirects that aren't triggered
// by user interaction. Uses `location.replace` so the kakaotalk:// URL
// doesn't land in history (cleaner back-navigation from the external
// browser).
export function escapeKakaoTalkInApp() {
  if (typeof window === "undefined") return;
  const url = window.location.href;
  window.location.replace(
    "kakaotalk://web/openExternal?url=" + encodeURIComponent(url),
  );
}
