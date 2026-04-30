// Forwards a JSON `DownloadAnalyzeExcelRequest` from Rust to the
// `window.ratel.spaces.apps.analyzes.downloadExcel` helper registered
// by `app/ratel/js/src/ratel-app-shell/spaces/apps/analyzes.js`.
//
// Returns `true` on success; sends `null` on missing fn or thrown
// error so Rust can surface a typed `SpaceAppError::ExcelExportFailed`.
const req = await dioxus.recv();
try {
  const fn = window?.ratel?.spaces?.apps?.analyzes?.downloadExcel;
  if (typeof fn !== "function") {
    dioxus.send(null);
  } else {
    await fn(req);
    dioxus.send(true);
  }
} catch (e) {
  console.error("downloadExcel failed", e);
  dioxus.send(null);
}
