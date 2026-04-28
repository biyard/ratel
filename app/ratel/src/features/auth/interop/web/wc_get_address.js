try {
  const addr = await window.ratel.auth.wallet.getAddress();
  dioxus.send(addr === undefined ? null : addr);
} catch (e) {
  dioxus.send(null);
}
