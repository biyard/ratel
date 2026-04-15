(function () {
  window.ratel = window.ratel || {};
  window.ratel.teamArena = window.ratel.teamArena || {};

  function initStars() {
    var layers = document.querySelectorAll('.team-arena__bg-stars');
    layers.forEach(function (layer) {
      if (layer.dataset.starsBound) return;
      layer.dataset.starsBound = 'true';

      var count = 90;
      for (var i = 0; i < count; i++) {
        var s = document.createElement('div');
        s.className = 'team-arena__star';
        var size = Math.random() < 0.15
          ? 2 + Math.random() * 1.5
          : 1 + Math.random() * 0.8;
        s.style.width = size + 'px';
        s.style.height = size + 'px';
        s.style.left = Math.random() * 100 + '%';
        s.style.top = Math.random() * 100 + '%';
        s.style.animationDuration = (3 + Math.random() * 5).toFixed(2) + 's';
        s.style.animationDelay = (-Math.random() * 6).toFixed(2) + 's';
        if (Math.random() < 0.08) {
          s.style.background = '#fcb300';
          s.style.boxShadow = '0 0 6px rgba(252,179,0,0.5)';
        } else if (Math.random() < 0.10) {
          s.style.background = '#6eedd8';
          s.style.boxShadow = '0 0 4px rgba(110,237,216,0.4)';
        }
        layer.appendChild(s);
      }

      for (var j = 0; j < 2; j++) {
        var ss = document.createElement('div');
        ss.className = 'team-arena__shooting';
        ss.style.top = (10 + j * 38 + Math.random() * 10) + '%';
        ss.style.left = (Math.random() * 40) + '%';
        ss.style.animationDelay = (-Math.random() * 8 + (j * 3)).toFixed(2) + 's';
        layer.appendChild(ss);
      }
    });
  }

  window.ratel.teamArena.initStars = initStars;

  initStars();
  new MutationObserver(function () { initStars(); })
    .observe(document.body, { childList: true, subtree: true });
})();
