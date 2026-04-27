/* Analyze arena LIST — carousel.
 *
 * Same pattern as `pages/index/action_dashboard/script.js`:
 *   - Native horizontal scroll-snap on `.report-carousel__track`
 *     drives the actual scroll position; cards have
 *     `scroll-snap-align: center` so the closest card snaps to centre.
 *   - A `scroll` listener finds the card nearest the track centre,
 *     toggles `.is-active` on that card and the matching dot, and
 *     updates the prev/next disabled state. Dioxus owns NONE of this
 *     state — it just renders the structure.
 *   - Prev / next / dot click handlers are attached here (not on the
 *     Dioxus side) and call `scrollToCard(index)`.
 *   - Window-level keydown listener: ArrowLeft → previous card,
 *     ArrowRight → next card. Skipped while typing in inputs and
 *     no-ops when the carousel isn't in the DOM (other routes).
 *   - Wrapped in the standard MutationObserver bind-once pattern so
 *     it works under both SSR (defer-loaded) and CSR (Dioxus mounts
 *     the page after the script tag has already executed).
 */
(function () {
  function initCarousel() {
    var track = document.getElementById("report-track");
    if (!track || track.dataset.analyzeCarouselBound) return;
    track.dataset.analyzeCarouselBound = "true";

    var dotsContainer = document.getElementById("report-dots");
    var prevBtn = document.getElementById("report-prev");
    var nextBtn = document.getElementById("report-next");

    var activeIndex = 0;

    function getCards() {
      return Array.from(track.querySelectorAll(".report-carousel__slide"));
    }
    function getDots() {
      return dotsContainer
        ? Array.from(dotsContainer.querySelectorAll(".report-carousel__dot"))
        : [];
    }

    function updateArrowStates() {
      var count = getCards().length;
      if (prevBtn) prevBtn.disabled = activeIndex <= 0;
      if (nextBtn) nextBtn.disabled = activeIndex >= count - 1;
    }

    function updateActive() {
      var trackRect = track.getBoundingClientRect();
      var center = trackRect.left + trackRect.width / 2;
      var cards = getCards();
      if (cards.length === 0) return;

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

      activeIndex = closest;
      cards.forEach(function (c, i) {
        c.classList.toggle("is-active", i === activeIndex);
      });

      var dots = getDots();
      dots.forEach(function (d, i) {
        d.classList.toggle("active", i === activeIndex);
        if (i === activeIndex) {
          d.setAttribute("data-active", "true");
        } else {
          d.removeAttribute("data-active");
        }
      });

      updateArrowStates();
    }

    function scrollToCard(index, behavior) {
      var cards = getCards();
      if (index < 0 || index >= cards.length) return;
      var card = cards[index];
      var trackRect = track.getBoundingClientRect();
      var cardRect = card.getBoundingClientRect();
      // Layout not ready (CSR navigation can fire init while the
      // browser is still resolving styles). Bail and let the next
      // tick / ResizeObserver retry.
      if (trackRect.width === 0 || cardRect.width === 0) return;
      var offset =
        cardRect.left -
        trackRect.left +
        track.scrollLeft -
        trackRect.width / 2 +
        cardRect.width / 2;
      track.scrollTo({ left: offset, behavior: behavior || "auto" });
    }

    // Expose a re-snap helper on the track so the global mutation
    // observer can re-centre after CSR navigation without re-binding.
    track.__analyzeRecentre = function () {
      scrollToCard(activeIndex);
      updateActive();
    };

    // Bind prev/next buttons.
    if (prevBtn) {
      prevBtn.addEventListener("click", function () {
        scrollToCard(activeIndex - 1, "smooth");
      });
    }
    if (nextBtn) {
      nextBtn.addEventListener("click", function () {
        scrollToCard(activeIndex + 1, "smooth");
      });
    }
    // Bind dot clicks — index matches DOM order.
    getDots().forEach(function (dot, i) {
      dot.addEventListener("click", function () {
        scrollToCard(i, "smooth");
      });
    });

    // Scroll-driven active highlight.
    track.addEventListener("scroll", updateActive, { passive: true });
    window.addEventListener("resize", function () {
      // After resize, the slide width may have changed (mobile media
      // queries). Re-snap to the active card so it stays centred.
      scrollToCard(activeIndex);
    });

    // Re-centre whenever the viewport / slide width settles. Fires
    // immediately after first layout AND every time SPA navigation
    // remounts the page — the track's clientWidth changes from 0 to
    // its real value, ResizeObserver picks that up and we re-snap.
    if (typeof ResizeObserver !== "undefined") {
      var ro = new ResizeObserver(function () {
        scrollToCard(activeIndex);
        updateActive();
      });
      ro.observe(track);
    }

    // Belt-and-suspenders: also schedule re-snaps at staggered ticks
    // so a slow first paint doesn't leave the carousel mis-centred.
    requestAnimationFrame(function () {
      scrollToCard(0);
      requestAnimationFrame(function () {
        scrollToCard(0);
        updateActive();
      });
    });
    setTimeout(function () {
      scrollToCard(activeIndex);
      updateActive();
    }, 100);
    setTimeout(function () {
      scrollToCard(activeIndex);
      updateActive();
    }, 300);
  }

  // Window-level arrow-key navigation (bind once, page-scoped via
  // checking the buttons exist before acting).
  if (!window.__ratelAnalyzeKeysBound) {
    window.__ratelAnalyzeKeysBound = true;
    window.addEventListener("keydown", function (e) {
      if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;

      // Don't hijack arrows while typing.
      var t = e.target;
      if (t) {
        var tag = (t.tagName || "").toLowerCase();
        if (tag === "input" || tag === "textarea" || tag === "select") return;
        if (t.isContentEditable) return;
      }

      var prev = document.getElementById("report-prev");
      var next = document.getElementById("report-next");
      if (!prev || !next) return; // carousel not on this page

      if (e.key === "ArrowLeft" && !prev.disabled) {
        e.preventDefault();
        prev.click();
      } else if (e.key === "ArrowRight" && !next.disabled) {
        e.preventDefault();
        next.click();
      }
    });
  }

  // Try immediately (works for SSR with defer).
  initCarousel();
  // Also observe for CSR rendering. Two cases:
  //   (a) Track is fresh (no `analyzeCarouselBound` flag) → init it.
  //   (b) Track was previously bound but the page just re-mounted
  //       (user navigated detail → list); the bind flag persists on
  //       the new DOM node only if Dioxus re-uses it. Either way,
  //       call the recentre helper if exposed so the carousel snaps
  //       to centre after the layout settles.
  new MutationObserver(function () {
    var track = document.getElementById("report-track");
    if (!track) return;
    if (!track.dataset.analyzeCarouselBound) {
      initCarousel();
    } else if (typeof track.__analyzeRecentre === "function") {
      requestAnimationFrame(track.__analyzeRecentre);
    }
  }).observe(document.body, { childList: true, subtree: true });
})();
