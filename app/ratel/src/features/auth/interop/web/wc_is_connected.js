try {
  const connected = window.ratel.auth.wallet.isConnected();
  dioxus.send(!!connected);
} catch (e) {
  dioxus.send(false);
}
