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

  // ── Fly-to-archive animation ────────────────────
  function flyToArchive(actionId) {
    var track = document.getElementById("carousel-track");
    var card = document.querySelector('[data-testid="quest-card-' + actionId + '"]');
    var archiveBtn = document.querySelector('[data-testid="btn-archive"]');
    var dotsContainer = document.getElementById("carousel-dots");
    if (!card || !archiveBtn || !track) return;

    // Find the index of the card being archived and determine the next card
    var allCards = Array.from(track.querySelectorAll(".quest-card"));
    var cardIndex = allCards.indexOf(card);
    var nextIndex = -1;
    // Prefer the next card; fall back to previous
    if (cardIndex + 1 < allCards.length) {
      nextIndex = cardIndex + 1;
    } else if (cardIndex - 1 >= 0) {
      nextIndex = cardIndex - 1;
    }

    var cardRect = card.getBoundingClientRect();
    var archiveRect = archiveBtn.getBoundingClientRect();
    var targetX = archiveRect.left + archiveRect.width / 2;
    var targetY = archiveRect.top + archiveRect.height / 2;

    // Clone card as fixed ghost for fly animation
    var ghost = card.cloneNode(true);
    ghost.style.position = "fixed";
    ghost.style.left = cardRect.left + "px";
    ghost.style.top = cardRect.top + "px";
    ghost.style.width = cardRect.width + "px";
    ghost.style.height = cardRect.height + "px";
    ghost.style.zIndex = "200";
    ghost.style.margin = "0";
    ghost.style.pointerEvents = "none";
    ghost.style.opacity = "1";
    ghost.style.transform = "scale(1)";
    ghost.style.filter = "blur(0)";
    ghost.style.transition = "all 0.7s cubic-bezier(0.4, 0, 0.2, 1)";
    ghost.style.borderRadius = "20px";
    ghost.style.overflow = "hidden";
    document.body.appendChild(ghost);

    // Hide original card
    card.style.transition = "opacity 0.3s ease";
    card.style.opacity = "0";
    card.style.pointerEvents = "none";

    // Trigger fly
    requestAnimationFrame(function () {
      requestAnimationFrame(function () {
        ghost.style.left = targetX - 20 + "px";
        ghost.style.top = targetY - 20 + "px";
        ghost.style.width = "40px";
        ghost.style.height = "40px";
        ghost.style.opacity = "0";
        ghost.style.borderRadius = "10px";
        ghost.style.transform = "scale(0.1)";
        ghost.style.filter = "blur(8px)";
      });
    });

    // Credits fly animation
    var credits = card.getAttribute("data-credits");
    if (credits && parseInt(credits, 10) > 0) {
      var pointsEl = document.createElement("span");
      pointsEl.className = "credits-fly-anim";
      pointsEl.textContent = "+" + credits + " CR";
      pointsEl.style.left = cardRect.left + cardRect.width / 2 + "px";
      pointsEl.style.top = cardRect.top + cardRect.height / 3 + "px";
      document.body.appendChild(pointsEl);
      setTimeout(function () {
        pointsEl.remove();
      }, 1300);
    }

    // Flash archive button
    setTimeout(function () {
      archiveBtn.classList.add("archive-btn--flash");
      setTimeout(function () {
        archiveBtn.classList.remove("archive-btn--flash");
      }, 600);
    }, 500);

    // After fly completes: scroll to next card and highlight it
    setTimeout(function () {
      ghost.remove();

      if (nextIndex < 0) return;
      // Scroll to next visible card (skip hidden ones)
      var visibleCards = Array.from(track.querySelectorAll(".quest-card")).filter(function (c) {
        return c.style.opacity !== "0";
      });
      // Find the next card in the visible list
      var nextCard = allCards[nextIndex];
      var visibleIndex = visibleCards.indexOf(nextCard);
      if (visibleIndex < 0 && visibleCards.length > 0) visibleIndex = 0;
      if (visibleIndex < 0) return;

      var targetCard = visibleCards[visibleIndex];
      var trackRect = track.getBoundingClientRect();
      var nextRect = targetCard.getBoundingClientRect();
      var scrollOffset =
        nextRect.left - trackRect.left + track.scrollLeft -
        trackRect.width / 2 + nextRect.width / 2;
      track.scrollTo({ left: scrollOffset, behavior: "smooth" });

      // Highlight the next card and update dots
      setTimeout(function () {
        visibleCards.forEach(function (c, i) {
          c.classList.toggle("active", i === visibleIndex);
        });
        if (dotsContainer) {
          var activeType = targetCard.dataset.type || "";
          var dots = Array.from(dotsContainer.querySelectorAll(".carousel-dot"));
          dots.forEach(function (d, i) {
            d.classList.remove("active", "active--poll", "active--discuss", "active--quiz", "active--follow");
            if (i === visibleIndex) d.classList.add("active", "active--" + activeType);
          });
        }
      }, 300);
    }, 800);
  }

  // Observe data-archive-action attribute changes on carousel-track
  var archiveObserver = new MutationObserver(function () {
    var track = document.getElementById("carousel-track");
    if (!track) return;
    var actionId = track.getAttribute("data-archive-action");
    if (actionId && actionId.length > 0 && !track.dataset.archiving) {
      track.dataset.archiving = "true";
      flyToArchive(actionId);
      setTimeout(function () {
        delete track.dataset.archiving;
      }, 1500);
    }
  });
  archiveObserver.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ["data-archive-action"],
  });
})();
