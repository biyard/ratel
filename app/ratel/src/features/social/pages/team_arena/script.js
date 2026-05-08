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

  // ── Post-card carousel: toggle .active on the centred card so its
  // CSS rule lifts opacity/blur back to readable. Without this the
  // default `.team-arena .post-card` state (opacity 0.30 + blur 5px)
  // makes every card a ghost. Mirrors the action-dashboard carousel.
  function initPostCarousel() {
    var tracks = document.querySelectorAll('.team-arena .carousel-track');
    tracks.forEach(function (track) {
      if (track.dataset.postCarouselBound) return;
      track.dataset.postCarouselBound = 'true';

      function updateActive() {
        var cards = Array.from(track.querySelectorAll('.post-card'));
        if (cards.length === 0) return;
        var trackRect = track.getBoundingClientRect();
        var center = trackRect.left + trackRect.width / 2;
        var closest = 0;
        var closestDist = Infinity;
        cards.forEach(function (card, i) {
          var rect = card.getBoundingClientRect();
          var dist = Math.abs(center - (rect.left + rect.width / 2));
          if (dist < closestDist) {
            closestDist = dist;
            closest = i;
          }
        });
        cards.forEach(function (c, i) {
          c.classList.toggle('active', i === closest);
        });
      }

      track.addEventListener('scroll', updateActive, { passive: true });
      // Initial pass + small retries — Dioxus may insert post-cards after
      // this script runs (CSR / hydration window), so a single pass on
      // bind would leave every card ghosted until the user touches the
      // scroll. The retries keep the active class in sync once cards
      // arrive.
      setTimeout(updateActive, 50);
      setTimeout(updateActive, 200);
      setTimeout(updateActive, 600);
      new MutationObserver(updateActive)
        .observe(track, { childList: true });
    });
  }

  window.ratel.teamArena.initPostCarousel = initPostCarousel;

  initStars();
  initPostCarousel();
  new MutationObserver(function () {
    initStars();
    initPostCarousel();
  }).observe(document.body, { childList: true, subtree: true });
})();
