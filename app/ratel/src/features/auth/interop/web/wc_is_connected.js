try {
  const connected = window.walletConnect.isConnected();
  dioxus.send(!!connected);
} catch (e) {
  dioxus.send(false);
}
