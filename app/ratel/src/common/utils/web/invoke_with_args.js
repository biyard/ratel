let { method, args } = await dioxus.recv();

// Await in case the dispatched handler is async (e.g. firebase signIn).
// Without await the bridge sends the raw Promise across `dioxus.send`,
// which deserializes as `{}` on the Rust side and surfaces as a panic.
let res;
try {
  res = await window.ratel.invoke(method, args);
} catch (e) {
  console.error(`${method} threw`, e);
  res = null;
}

dioxus.send(res);
