(function () {
  function initCarousel() {
    var track = document.getElementById("carousel-track");
    if (!track || track.dataset.carouselBound) return;
    track.dataset.carouselBound = "true";

    var cards = Array.from(track.querySelectorAll(".quest-card"));
    var dotsContainer = document.getElementById("carousel-dots");
    if (!dotsContainer) return;

    var activeIndex = 0;

    // Bind dot clicks
    var dots = Array.from(dotsContainer.querySelectorAll(".carousel-dot"));
    dots.forEach(function (dot, i) {
      dot.addEventListener("click", function () {
        scrollToCard(i);
      });
    });

    function updateActive() {
      var trackRect = track.getBoundingClientRect();
      var center = trackRect.left + trackRect.width / 2;
      var closest = 0;
      var closestDist = Infinity;
      var currentCards = Array.from(track.querySelectorAll(".quest-card"));

      currentCards.forEach(function (card, i) {
        var rect = card.getBoundingClientRect();
        var dist = Math.abs(center - (rect.left + rect.width / 2));
        if (dist < closestDist) {
          closestDist = dist;
          closest = i;
        }
      });

      if (currentCards.length === 0) return;
      activeIndex = closest;
      var activeType = currentCards[activeIndex]
        ? currentCards[activeIndex].dataset.type
        : "";

      currentCards.forEach(function (c, i) {
        c.classList.toggle("active", i === activeIndex);
      });

      var currentDots = Array.from(
        dotsContainer.querySelectorAll(".carousel-dot"),
      );
      currentDots.forEach(function (d, i) {
        d.classList.remove(
          "active",
          "active--poll",
          "active--discuss",
          "active--quiz",
          "active--follow",
        );
        if (i === activeIndex)
          d.classList.add("active", "active--" + activeType);
      });
    }

    function scrollToCard(index) {
      var currentCards = Array.from(track.querySelectorAll(".quest-card"));
      if (index >= currentCards.length) return;
      var card = currentCards[index];
      var trackRect = track.getBoundingClientRect();
      var cardRect = card.getBoundingClientRect();
      var offset =
        cardRect.left -
        trackRect.left +
        track.scrollLeft -
        trackRect.width / 2 +
        cardRect.width / 2;
      track.scrollTo({ left: offset, behavior: "smooth" });
    }

    track.addEventListener("scroll", updateActive, { passive: true });

    // Initial highlight
    requestAnimationFrame(function () {
      scrollToCard(0);
      setTimeout(updateActive, 100);
    });
  }

  // Try immediately (works for SSR with defer)
  initCarousel();

  // Also observe for CSR rendering (Dioxus adds elements after script runs)
  var observer = new MutationObserver(function () {
    var track = document.getElementById("carousel-track");
    if (track && !track.dataset.carouselBound) {
      initCarousel();
    }
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();
