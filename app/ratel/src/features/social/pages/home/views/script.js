(function () {
  window.ratel = window.ratel || {};
  window.ratel.teamHomeArena = window.ratel.teamHomeArena || {};

  function closestToCenter(track, cards) {
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
    return closest;
  }

  function bindTrack(track) {
    if (!track || track.dataset.carouselBound) return;
    track.dataset.carouselBound = 'true';

    var dotsContainer = track.parentElement
      ? track.parentElement.querySelector('.carousel-dots')
      : null;

    function sync() {
      var cards = Array.from(track.querySelectorAll('.post-card'));
      if (!cards.length) return;

      // Rebuild dots when card count changes
      if (dotsContainer) {
        var existing = dotsContainer.querySelectorAll('.carousel-dot');
        if (existing.length !== cards.length) {
          dotsContainer.innerHTML = '';
          cards.forEach(function (_, i) {
            var dot = document.createElement('button');
            dot.className = 'carousel-dot';
            dot.type = 'button';
            dot.addEventListener('click', function () {
              scrollToCard(track, cards, i);
            });
            dotsContainer.appendChild(dot);
          });
        }
      }

      var dots = dotsContainer
        ? Array.from(dotsContainer.querySelectorAll('.carousel-dot'))
        : [];

      var active = closestToCenter(track, cards);
      cards.forEach(function (c, i) {
        c.classList.toggle('active', i === active);
      });
      dots.forEach(function (d, i) {
        d.classList.toggle('active', i === active);
      });
    }

    track.addEventListener('scroll', sync, { passive: true });
    window.addEventListener('resize', sync);

    // Observe card list changes so sync runs after Dioxus re-renders
    var observer = new MutationObserver(function () {
      sync();
    });
    observer.observe(track, { childList: true, subtree: false });

    requestAnimationFrame(sync);
  }

  function scrollToCard(track, cards, i) {
    var card = cards[i];
    if (!card) return;
    var trackRect = track.getBoundingClientRect();
    var cardRect = card.getBoundingClientRect();
    var offset =
      cardRect.left - trackRect.left - (trackRect.width - cardRect.width) / 2;
    track.scrollBy({ left: offset, behavior: 'smooth' });
  }

  function init() {
    document
      .querySelectorAll('.carousel-track')
      .forEach(function (t) { bindTrack(t); });
  }

  window.ratel.teamHomeArena.init = init;

  init();
  new MutationObserver(function () { init(); })
    .observe(document.body, { childList: true, subtree: true });
})();
