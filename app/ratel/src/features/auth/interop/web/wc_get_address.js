try {
  const addr = await window.walletConnect.getAddress();
  dioxus.send(addr === undefined ? null : addr);
} catch (e) {
  dioxus.send(null);
}
