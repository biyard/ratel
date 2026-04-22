// Smooth-scroll the action editor's `.pager` to a specific page index.
// Called from Rust (wasm_bindgen) when the user clicks Previous/Next in
// `ActionEditFooter`. The pager is `scroll-snap-type: x mandatory`, so
// any offset within a page snaps to it; we compute the canonical page
// boundary by `index * pager.clientWidth` and let the browser snap.
(function () {
  function goToPage(index) {
    var pager = document.querySelector(".pager");
    if (!pager) return;
    var width = pager.clientWidth;
    if (!width) return;
    pager.scrollTo({ left: index * width, behavior: "smooth" });
  }

  window.ratel = window.ratel || {};
  window.ratel.actionEditor = window.ratel.actionEditor || {};
  window.ratel.actionEditor.goToPage = goToPage;
})();
