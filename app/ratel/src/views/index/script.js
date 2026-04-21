(function () {
  function init() {
    var track = document.getElementById("home-carousel-track");
    if (!track || track.dataset.homeArenaBound) return;
    track.dataset.homeArenaBound = "true";

    var dotsContainer = document.getElementById("home-carousel-dots");
    if (!dotsContainer) return;

    function currentCards() {
      return Array.from(track.querySelectorAll(".space-card"));
    }
    function currentDots() {
      return Array.from(dotsContainer.querySelectorAll(".carousel-dot"));
    }

    function bindDots() {
      currentDots().forEach(function (dot, i) {
        if (dot.dataset.bound) return;
        dot.dataset.bound = "true";
        dot.addEventListener("click", function () { scrollToCard(i); });
      });
    }

    function updateActive() {
      var cards = currentCards();
      if (cards.length === 0) return;
      var trackRect = track.getBoundingClientRect();
      var center = trackRect.left + trackRect.width / 2;
      var closest = 0;
      var closestDist = Infinity;
      cards.forEach(function (card, i) {
        var rect = card.getBoundingClientRect();
        var dist = Math.abs(center - (rect.left + rect.width / 2));
        if (dist < closestDist) { closestDist = dist; closest = i; }
      });
      var activeHeat = cards[closest].dataset.heat || "";
      cards.forEach(function (c, i) { c.classList.toggle("active", i === closest); });
      currentDots().forEach(function (d, i) {
        d.classList.remove("active", "active--blazing", "active--trending", "active--rising");
        if (i === closest) d.classList.add("active", "active--" + activeHeat);
      });
    }

    function scrollToCard(index) {
      var cards = currentCards();
      if (index < 0 || index >= cards.length) return;
      var card = cards[index];
      var trackRect = track.getBoundingClientRect();
      var cardRect = card.getBoundingClientRect();
      var offset = cardRect.left - trackRect.left + track.scrollLeft
        - trackRect.width / 2 + cardRect.width / 2;
      track.scrollTo({ left: offset, behavior: "smooth" });
    }

    bindDots();
    track.addEventListener("scroll", updateActive, { passive: true });
    window.addEventListener("resize", updateActive);

    // Re-run active detection when Dioxus swaps the card list (tab change).
    // Without this, newly mounted cards keep the default blurred style because
    // no scroll event fires to trigger updateActive().
    new MutationObserver(function () {
      bindDots();
      requestAnimationFrame(function () {
        scrollToCard(0);
        setTimeout(updateActive, 50);
      });
    }).observe(track, { childList: true });

    requestAnimationFrame(function () {
      scrollToCard(0);
      setTimeout(updateActive, 100);
    });
  }

  init();

  new MutationObserver(function () {
    var track = document.getElementById("home-carousel-track");
    if (track && !track.dataset.homeArenaBound) init();
  }).observe(document.body, { childList: true, subtree: true });
})();
