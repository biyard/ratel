let data = await dioxus.recv();

window.walletConnect.initialize(
  data.projectId,
  data.appName,
  data.appDescription,
  data.appUrl,
);
