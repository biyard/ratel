try {
  await window.ratel.auth.wallet.openWalletApp();
  dioxus.send(true);
} catch (e) {
  dioxus.send(false);
}
