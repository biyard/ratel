const url = await dioxus.recv();
if (typeof url === "string" && url.length > 0) {
  window.location.href = url;
}
