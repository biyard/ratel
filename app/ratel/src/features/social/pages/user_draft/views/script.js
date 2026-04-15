(function () {
  window.ratel = window.ratel || {};
  window.ratel.drafts = window.ratel.drafts || {};

  function closeAllCardMenus(except) {
    document.querySelectorAll('.drafts-arena .draft-card[data-menu-open="true"]').forEach(function (card) {
      if (card !== except) card.setAttribute('data-menu-open', 'false');
    });
  }

  function closeSort(except) {
    var el = document.getElementById('drafts-sort');
    if (el && el !== except && el.getAttribute('data-open') === 'true') {
      el.setAttribute('data-open', 'false');
    }
  }

  function onDocClick(e) {
    var sort = document.getElementById('drafts-sort');
    if (sort && !sort.contains(e.target)) closeSort(null);
    if (!e.target.closest('.drafts-arena .draft-card')) closeAllCardMenus(null);
  }

  function init() {
    if (!document.body.dataset.ratelDraftsBound) {
      document.body.dataset.ratelDraftsBound = 'true';
      document.addEventListener('click', onDocClick);
    }
  }

  init();
  new MutationObserver(init).observe(document.body, { childList: true, subtree: true });
})();
