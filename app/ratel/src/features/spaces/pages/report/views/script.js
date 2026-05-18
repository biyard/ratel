// Reports list carousel — direct port of the JS block in
// `assets/design/reports/reports-list.html`. Only the carousel slice
// (active-card detection + dot navigation) is ported here; filter chips
// are handled in Dioxus instead. A MutationObserver re-binds on CSR
// remounts so navigating into the page from another route still wires
// up the scroll/resize handlers.

(function () {
  function init(root) {
    if (!root || root.dataset.bound === "true") return;
    var track = root.querySelector("#carousel-track");
    var dotsContainer = root.querySelector("#carousel-dots");
    if (!track || !dotsContainer) return;
    root.dataset.bound = "true";

    var activeIndex = 0;

    function getCards() {
      return Array.prototype.slice.call(
        track.querySelectorAll(".report-card:not([hidden])")
      );
    }
    function getDots() {
      return Array.prototype.slice.call(
        dotsContainer.querySelectorAll(".carousel-dot")
      );
    }

    function rebuildDots() {
      var cards = getCards();
      dotsContainer.innerHTML = "";
      cards.forEach(function (c, i) {
        var dot = document.createElement("button");
        dot.className = "carousel-dot";
        if (c.dataset.kind === "create") dot.dataset.kind = "create";
        dot.addEventListener("click", function () { scrollToCard(i); });
        dotsContainer.appendChild(dot);
      });
    }

    function updateActive() {
      var cards = getCards();
      if (!cards.length) return;
      var trackRect = track.getBoundingClientRect();
      var center = trackRect.left + trackRect.width / 2;
      var closest = 0;
      var closestDist = Infinity;
      cards.forEach(function (card, i) {
        var rect = card.getBoundingClientRect();
        var dist = Math.abs(center - (rect.left + rect.width / 2));
        if (dist < closestDist) { closestDist = dist; closest = i; }
      });
      activeIndex = Math.min(closest, cards.length - 1);
      cards.forEach(function (c, i) {
        c.classList.toggle("active", i === activeIndex);
      });
      getDots().forEach(function (d, i) {
        d.classList.remove("active", "active--create");
        if (i === activeIndex) {
          d.classList.add("active");
          if (d.dataset.kind === "create") d.classList.add("active--create");
        }
      });
    }

    function scrollToCard(i) {
      var cards = getCards();
      var card = cards[i];
      if (!card) return;
      var trackRect = track.getBoundingClientRect();
      var cardRect = card.getBoundingClientRect();
      var offset =
        cardRect.left - trackRect.left -
        (trackRect.width - cardRect.width) / 2;
      track.scrollBy({ left: offset, behavior: "smooth" });
    }

    // Clicking an off-center card snaps it to center first; only when it
    // becomes the active card does the underlying link navigate.
    track.addEventListener("click", function (e) {
      var card = e.target.closest(".report-card");
      if (!card) return;
      var cards = getCards();
      var i = cards.indexOf(card);
      if (i !== -1 && i !== activeIndex) {
        e.preventDefault();
        scrollToCard(i);
      }
    });

    rebuildDots();
    track.addEventListener("scroll", updateActive, { passive: true });
    window.addEventListener("resize", updateActive);
    requestAnimationFrame(updateActive);

    // Filter chips mutate `hidden` from the Dioxus side; observe DOM
    // changes within the track and rebuild dots / re-pick active card.
    var mo = new MutationObserver(function () {
      rebuildDots();
      requestAnimationFrame(updateActive);
    });
    mo.observe(track, {
      childList: true,
      subtree: true,
      attributes: true,
      attributeFilter: ["hidden"],
    });
  }

  function tryInit() {
    var root = document.querySelector(".reports-arena");
    if (root) init(root);
  }
  tryInit();
  // CSR navigation — the reports-arena mounts after this script runs.
  new MutationObserver(function () { tryInit(); }).observe(document.body, {
    childList: true,
    subtree: true,
  });
})();
