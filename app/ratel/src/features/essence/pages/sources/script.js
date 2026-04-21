/* Essence Sources — keyboard shortcut (⌘K / Ctrl+K focuses search input).
   Dioxus owns filter/selection/toggle state through signals; this script is
   only for DOM behavior that RSX can't express declaratively (global
   keyboard capture outside the input element). */

(function () {
  function init() {
    if (document.body.dataset.essenceSourcesBound) return;
    document.body.dataset.essenceSourcesBound = 'true';

    document.addEventListener('keydown', function (e) {
      var isMac = navigator.platform.toUpperCase().indexOf('MAC') !== -1;
      var cmd = isMac ? e.metaKey : e.ctrlKey;
      if (cmd && e.key.toLowerCase() === 'k') {
        var input = document.querySelector('[data-essence-search-input]');
        if (input) {
          e.preventDefault();
          input.focus();
          input.select();
        }
      }
    });
  }

  init();
  new MutationObserver(function () { init(); })
    .observe(document.body, { childList: true, subtree: true });
})();
