try {
  await window.walletConnect.disconnect();
  dioxus.send(true);
} catch (e) {
  dioxus.send(false);
}
