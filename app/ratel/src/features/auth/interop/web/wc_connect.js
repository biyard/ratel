try {
  const r = await window.ratel.auth.wallet.connect();
  dioxus.send(r);
} catch (e) {
  dioxus.send(null);
}
