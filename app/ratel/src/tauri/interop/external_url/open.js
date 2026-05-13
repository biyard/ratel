// Driver for crate::tauri::interop::external_url::open.
// Receives ExternalUrlRequest as JSON, calls Tauri, sends back
// { "ok": ExternalUrlResponse } or { "err": ExternalUrlError }.

(async () => {
  try {
    const req = await dioxus.recv();
    if (!window.__TAURI__ || !window.__TAURI__.core || !window.__TAURI__.core.invoke) {
      dioxus.send({ err: { OpenerFailed: "window.__TAURI__ unavailable" } });
      return;
    }
    const res = await window.__TAURI__.core.invoke("open_external_url", { req });
    // The #[tauri::command] returns ExternalUrlResponse directly on success and
    // surfaces errors via Tauri's error channel — so a thrown exception means
    // ExternalUrlError. A normal return is the success DTO.
    dioxus.send({ ok: res });
  } catch (e) {
    // Tauri serializes #[tauri::command] errors as plain strings or the error's
    // serde form. Try to parse it as ExternalUrlError; fall back to OpenerFailed.
    const msg = (e && e.message) ? e.message : String(e);
    let err;
    try {
      err = (typeof e === "object" && e !== null && (e.InvalidUrl || e.OpenerFailed))
        ? e
        : { OpenerFailed: msg };
    } catch (_) {
      err = { OpenerFailed: msg };
    }
    dioxus.send({ err });
  }
})();
