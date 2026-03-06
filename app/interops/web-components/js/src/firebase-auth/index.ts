import { initializeApp, type FirebaseApp } from "firebase/app";
import {
  getAuth,
  signInWithPopup,
  GoogleAuthProvider,
  type Auth,
} from "firebase/auth";

class FirebaseAuthElement extends HTMLElement {
  private _app: FirebaseApp | null = null;
  private _auth: Auth | null = null;
  private _provider: GoogleAuthProvider | null = null;

  static get observedAttributes(): string[] {
    return ["api-key", "auth-domain", "project-id"];
  }

  connectedCallback(): void {
    this.style.display = "contents";
    this.addEventListener("click", this._handleClick);
    this._initFirebase();
  }

  disconnectedCallback(): void {
    this.removeEventListener("click", this._handleClick);
  }

  attributeChangedCallback(): void {
    this._initFirebase();
  }

  private _initFirebase(): void {
    const apiKey = this.getAttribute("api-key");
    const authDomain = this.getAttribute("auth-domain");
    const projectId = this.getAttribute("project-id");

    if (!apiKey || !authDomain || !projectId) return;

    try {
      this._app = initializeApp(
        { apiKey, authDomain, projectId },
        `firebase-auth-${projectId}`,
      );
      this._auth = getAuth(this._app);
      this._provider = new GoogleAuthProvider();
    } catch {
      // App may already be initialized with this name; reuse it
      try {
        this._app = initializeApp(
          { apiKey, authDomain, projectId },
          `firebase-auth-${projectId}-${Date.now()}`,
        );
        this._auth = getAuth(this._app);
        this._provider = new GoogleAuthProvider();
      } catch (e) {
        console.error("Firebase init failed:", e);
      }
    }
  }

  private _handleClick = async (): Promise<void> => {
    if (!this._auth || !this._provider) {
      this._dispatch({ err: "Firebase not initialized" });
      return;
    }

    try {
      const result = await signInWithPopup(this._auth, this._provider);
      const user = result.user;
      const credential = GoogleAuthProvider.credentialFromResult(result);
      const accessToken = credential?.accessToken ?? "";
      const idToken = await user.getIdToken();

      this._dispatch({
        ok: {
          access_token: accessToken,
          id_token: idToken,
          email: user.email,
          display_name: user.displayName,
          photo_url: user.photoURL,
        },
      });
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : String(e);
      this._dispatch({ err: message });
    }
  };

  private _dispatch(detail: object): void {
    this.dispatchEvent(
      new CustomEvent("change", {
        detail: JSON.stringify(detail),
        bubbles: true,
        composed: true,
      }),
    );
  }
}

if (!customElements.get("firebase-auth")) {
  customElements.define("firebase-auth", FirebaseAuthElement);
}
