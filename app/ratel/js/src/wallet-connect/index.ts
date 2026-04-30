import { SignClient } from "@walletconnect/sign-client";
import { createAppKit } from "@reown/appkit";
import { mainnet, kaia } from "@reown/appkit/networks";

let config: any = null;
let client: any = null;
let appKit: any = null;
let initPromise: any = null;
let activeSession: any = null;
let connectedAddress: any = null;
let connectedChainId: any = null;

export function initialize(
  projectId: string,
  appName: string,
  appDescription: string,
  appUrl: string,
) {
  if (config || typeof window === "undefined") return;

  config = {
    projectId,
    metadata: {
      name: appName || "Ratel",
      description: appDescription || "Ratel",
      url: appUrl || window.location.origin,
      icons: ["https://metadata.ratel.foundation/logos/logo-symbol.png"],
    },
  };
  console.log("[wallet] config saved", projectId);
}

async function getClient() {
  if (client) return { client, appKit };
  if (!config) throw new Error("WalletConnect not initialized");

  if (!initPromise) {
    initPromise = (async () => {
      client = await SignClient.init({
        projectId: config.projectId,
        metadata: config.metadata,
      });

      appKit = createAppKit({
        projectId: config.projectId,
        metadata: config.metadata,
        networks: [mainnet, kaia],
        features: {
          analytics: false,
          email: false,
          socials: false,
        },
        featuredWalletIds: [],
      });

      console.log("[wallet] SignClient + AppKit initialized");
    })();
  }

  await initPromise;
  return { client, appKit };
}

// Connect wallet — returns { address, chain_id }, keeps session alive
export async function connect() {
  const { client, appKit } = await getClient();

  const { uri: wcUri, approval } = await client.connect({
    optionalNamespaces: {
      eip155: {
        chains: ["eip155:1", "eip155:8217"],
        methods: ["personal_sign"],
        events: ["chainChanged", "accountsChanged"],
      },
    },
  });

  // Watch for user closing the modal
  const modalClosed = new Promise((_, reject) => {
    const unsub = appKit.subscribeState((state: any) => {
      if (!state.open) {
        unsub();
        reject(new Error("User cancelled"));
      }
    });
  });

  if (wcUri) {
    await appKit.open({ uri: wcUri });
  }

  try {
    const session = await Promise.race([approval(), modalClosed]);
    console.log("[wallet] connected, session:", session.topic);

    const accounts = session.namespaces?.eip155?.accounts || [];
    if (accounts.length === 0) {
      throw new Error("No accounts in session");
    }

    // account format: "eip155:<chainId>:<address>"
    const parts = accounts[0].split(":");
    const address = parts[2];
    const chainId = parseInt(parts[1], 10);

    activeSession = session;
    connectedAddress = address;
    connectedChainId = chainId;

    console.log("[wallet] connected:", address, "chainId:", chainId);
    return { address, chain_id: chainId };
  } finally {
    await appKit.close();
  }
}

// Sign a message using the active session — returns signature string
export async function signMessage(message: any) {
  if (!activeSession || !connectedAddress) {
    throw new Error("No active wallet session. Call connect() first.");
  }

  const { client } = await getClient();

  const signature = await client.request({
    topic: activeSession.topic,
    chainId: `eip155:${connectedChainId}`,
    request: {
      method: "personal_sign",
      params: [message, connectedAddress],
    },
  });

  console.log("[wallet] message signed");
  return signature;
}

export async function getAddress() {
  return connectedAddress || null;
}

export async function disconnect() {
  if (activeSession) {
    try {
      const { client } = await getClient();
      await client.disconnect({
        topic: activeSession.topic,
        reason: { code: 6000, message: "User disconnected" },
      });
      console.log("[wallet] session disconnected");
    } catch (e) {
      console.warn("[wallet] session disconnect failed:", e);
    }
  }
  activeSession = null;
  connectedAddress = null;
  connectedChainId = null;
}

// Open the wallet app for pending sign request
// Tries session peer redirect, falls back to AppKit modal
export async function openWalletApp() {
  if (activeSession && activeSession.peer && activeSession.peer.metadata) {
    const meta = activeSession.peer.metadata;
    // Try native redirect first (works on mobile)
    if (meta.redirect && meta.redirect.native) {
      window.location.href = meta.redirect.native;
      return;
    }
    if (meta.redirect && meta.redirect.universal) {
      window.location.href = meta.redirect.universal;
      return;
    }
  }

  // Fallback: open AppKit modal which shows connected wallet
  const { appKit } = await getClient();
  if (appKit) {
    await appKit.open();
  }
}

export function isConnected() {
  return !!activeSession && !!connectedAddress;
}

if (typeof window !== "undefined") {
  window.walletConnect = {
    initialize,
    connect,
    disconnect,
    getAddress,
    openWalletApp,
    isConnected,
    signMessage,
  };
}
