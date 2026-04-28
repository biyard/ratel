const message = await dioxus.recv();
try {
  const sig = await window.walletConnect.signMessage(message);
  dioxus.send(sig);
} catch (e) {
  dioxus.send(null);
}
