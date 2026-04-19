(function () {
  window.ratel = window.ratel || {};
  window.ratel.postComposer = window.ratel.postComposer || {};

  // Lock/unlock body scroll when the mobile drawer is open.
  function syncBodyScroll() {
    var panel = document.getElementById('post-side-panel');
    if (!panel) return;
    var open = panel.getAttribute('data-open') === 'true';
    document.body.style.overflow = open ? 'hidden' : '';
  }

  // Close the "posting as" dropdown when the user clicks outside of it.
  function onDocClick(e) {
    var dropdown = document.getElementById('as-dropdown');
    if (!dropdown) return;
    if (dropdown.getAttribute('data-open') !== 'true') return;
    if (dropdown.contains(e.target)) return;
    var trigger = document.getElementById('as-dropdown-trigger');
    if (trigger) trigger.click();
  }

  function init() {
    var panel = document.getElementById('post-side-panel');
    if (panel && !panel.dataset.ratelBound) {
      panel.dataset.ratelBound = 'true';
      new MutationObserver(syncBodyScroll).observe(panel, {
        attributes: true,
        attributeFilter: ['data-open'],
      });
      syncBodyScroll();
    }

    if (!document.body.dataset.ratelComposerClick) {
      document.body.dataset.ratelComposerClick = 'true';
      document.addEventListener('click', onDocClick);
    }
  }

  init();
  new MutationObserver(init).observe(document.body, {
    childList: true,
    subtree: true,
  });
})();
