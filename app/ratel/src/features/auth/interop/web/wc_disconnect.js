try {
  await window.ratel.auth.wallet.disconnect();
  dioxus.send(true);
} catch (e) {
  dioxus.send(false);
}
