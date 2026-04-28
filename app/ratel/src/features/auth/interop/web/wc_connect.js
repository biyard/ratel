try {
  const r = await window.walletConnect.connect();
  dioxus.send(r);
} catch (e) {
  dioxus.send(null);
}
