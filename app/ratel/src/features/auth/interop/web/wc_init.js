let data = await dioxus.recv();

window.ratel.auth.wallet.initialize(
  data.projectId,
  data.appName,
  data.appDescription,
  data.appUrl,
);
