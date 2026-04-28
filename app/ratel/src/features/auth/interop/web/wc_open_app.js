try {
  await window.walletConnect.openWalletApp();
  dioxus.send(true);
} catch (e) {
  dioxus.send(false);
}
