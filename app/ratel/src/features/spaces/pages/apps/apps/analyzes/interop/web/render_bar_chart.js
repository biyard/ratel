const req = await dioxus.recv();
try {
  const fn = window?.ratel?.spaces?.apps?.analyzes?.renderBarChart;
  if (typeof fn !== "function") {
    dioxus.send(null);
  } else {
    fn(req);
    dioxus.send(true);
  }
} catch (e) {
  console.error("renderBarChart failed", e);
  dioxus.send(null);
}
