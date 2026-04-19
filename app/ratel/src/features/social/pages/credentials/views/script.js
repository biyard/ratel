(function () {
  window.ratel = window.ratel || {};
  window.ratel.credentials = window.ratel.credentials || {};

  function onCopyClick(e) {
    var btn = e.currentTarget;
    var holder = btn.parentElement;
    if (!holder) return;
    var valueEl = holder.querySelector('[data-did-value]');
    var value = valueEl ? valueEl.textContent.trim() : '';
    if (!value || !navigator.clipboard) return;
    navigator.clipboard.writeText(value).catch(function () {});
    btn.setAttribute('data-copied', 'true');
    setTimeout(function () {
      btn.removeAttribute('data-copied');
    }, 1200);
  }

  window.ratel.credentials.export_vc = function (json) {
    try {
      var blob = new Blob([json], { type: 'application/json' });
      var url = URL.createObjectURL(blob);
      var a = document.createElement('a');
      a.href = url;
      a.download = 'ratel-vc.json';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      setTimeout(function () { URL.revokeObjectURL(url); }, 1000);
    } catch (e) {
      if (navigator.clipboard) {
        navigator.clipboard.writeText(json).catch(function () {});
      }
    }
  };

  function init() {
    document.querySelectorAll('[data-did-copy]').forEach(function (btn) {
      if (btn.dataset.ratelBound) return;
      btn.dataset.ratelBound = 'true';
      btn.addEventListener('click', onCopyClick);
    });
  }

  init();
  new MutationObserver(init).observe(document.body, { childList: true, subtree: true });
})();
