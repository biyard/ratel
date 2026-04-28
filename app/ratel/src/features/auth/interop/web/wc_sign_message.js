const message = await dioxus.recv();
try {
  const sig = await window.ratel.auth.wallet.signMessage(message);
  dioxus.send(sig);
} catch (e) {
  dioxus.send(null);
}
