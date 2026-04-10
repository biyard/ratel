// Carousel
const track = document.getElementById("carousel-track");
const cards = Array.from(track.querySelectorAll(".quest-card"));
const dotsContainer = document.getElementById("carousel-dots");
let activeIndex = 0;

cards.forEach((_, i) => {
  const dot = document.createElement("button");
  dot.className = "carousel-dot";
  dot.dataset.type = cards[i].dataset.type;
  dot.addEventListener("click", () => scrollToCard(i));
  dotsContainer.appendChild(dot);
});
const dots = Array.from(dotsContainer.querySelectorAll(".carousel-dot"));

function updateActive() {
  const trackRect = track.getBoundingClientRect();
  const center = trackRect.left + trackRect.width / 2;
  let closest = 0,
    closestDist = Infinity;
  cards.forEach((card, i) => {
    const rect = card.getBoundingClientRect();
    const dist = Math.abs(center - (rect.left + rect.width / 2));
    if (dist < closestDist) {
      closestDist = dist;
      closest = i;
    }
  });
  activeIndex = closest;
  const activeType = cards[activeIndex].dataset.type;
  cards.forEach((c, i) => c.classList.toggle("active", i === activeIndex));
  dots.forEach((d, i) => {
    d.classList.remove(
      "active",
      "active--poll",
      "active--discuss",
      "active--quiz",
      "active--follow",
    );
    if (i === activeIndex) d.classList.add("active", "active--" + activeType);
  });
}

function scrollToCard(index) {
  const card = cards[index];
  const trackRect = track.getBoundingClientRect();
  const cardRect = card.getBoundingClientRect();
  const offset =
    cardRect.left -
    trackRect.left +
    track.scrollLeft -
    trackRect.width / 2 +
    cardRect.width / 2;
  track.scrollTo({ left: offset, behavior: "smooth" });
}

track.addEventListener("scroll", updateActive, { passive: true });
requestAnimationFrame(() => {
  scrollToCard(0);
  setTimeout(updateActive, 100);
});

// Archive panel
const archiveBtn = document.getElementById("archive-btn");
const archivePanel = document.getElementById("archive-panel");
const archiveClose = document.getElementById("archive-close");
archiveBtn.addEventListener("click", () =>
  archivePanel.classList.toggle("open"),
);
archiveClose.addEventListener("click", () =>
  archivePanel.classList.remove("open"),
);

// Follow button with points animation
function handleFollow(btn) {
  if (btn.dataset.followed === "true") return;
  btn.dataset.followed = "true";
  btn.textContent = "Following";

  // Animate points
  const points = btn.dataset.points || "10";
  const anim = document.createElement("span");
  anim.className = "points-anim";
  anim.textContent = "+" + points;
  btn.parentElement.appendChild(anim);
  setTimeout(function () {
    anim.remove();
  }, 1000);

  // Update follow count
  const card = btn.closest(".quest-card");
  const followBtns = card.querySelectorAll(".quest-follow-user__btn");
  const followedCount = Array.from(followBtns).filter(function (b) {
    return b.dataset.followed === "true";
  }).length;
  const totalCount = followBtns.length;
  const footerSpan = card.querySelector(".quest-card__footer span");
  if (footerSpan)
    footerSpan.textContent = followedCount + " / " + totalCount + " followed";

  // All followed: fly card to archive
  if (followedCount === totalCount) {
    setTimeout(function () {
      archiveCard(card);
    }, 600);
  }
}

function archiveCard(card) {
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
  ghost.style.zIndex = "100";
  ghost.style.margin = "0";
  ghost.style.pointerEvents = "none";
  ghost.style.opacity = "1";
  ghost.style.transform = "scale(1)";
  ghost.style.filter = "blur(0)";
  ghost.style.transition = "all 0.7s cubic-bezier(0.4, 0, 0.2, 1)";
  ghost.style.borderRadius = "20px";
  ghost.style.overflow = "hidden";
  document.body.appendChild(ghost);

  // Hide original
  card.style.transition = "none";
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

  // After animation: cleanup
  setTimeout(function () {
    ghost.remove();
    card.remove();

    // Rebuild carousel
    var remainingCards = Array.from(track.querySelectorAll(".quest-card"));
    cards.length = 0;
    cards.push.apply(cards, remainingCards);
    dotsContainer.textContent = "";
    cards.forEach(function (c, i) {
      var dot = document.createElement("button");
      dot.className = "carousel-dot";
      dot.dataset.type = c.dataset.type;
      dot.addEventListener("click", function () {
        scrollToCard(i);
      });
      dotsContainer.appendChild(dot);
    });
    dots.length = 0;
    dots.push.apply(
      dots,
      Array.from(dotsContainer.querySelectorAll(".carousel-dot")),
    );

    if (cards.length > 0) {
      var nextIndex = Math.min(activeIndex, cards.length - 1);
      scrollToCard(nextIndex);
      setTimeout(updateActive, 150);
    }

    // Flash archive button
    archiveBtn.classList.add("archive-btn--flash");
    setTimeout(function () {
      archiveBtn.classList.remove("archive-btn--flash");
    }, 600);

    // Bump count
    var countEl = archiveBtn.querySelector(".archive-btn__count");
    countEl.textContent = (parseInt(countEl.textContent, 10) || 0) + 1;

    // Add archive list entry via DOM methods
    var list = archivePanel.querySelector(".archive-panel__list");
    var item = document.createElement("div");
    item.className = "archive-item";

    var iconDiv = document.createElement("div");
    iconDiv.className = "archive-item__icon archive-item__icon--follow";
    iconDiv.appendChild(
      createSvg(
        "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2",
        "M9 7a4 4 0 1 0 0 0.01",
      ),
    );
    item.appendChild(iconDiv);

    var infoDiv = document.createElement("div");
    infoDiv.className = "archive-item__info";
    var titleEl = document.createElement("div");
    titleEl.className = "archive-item__title";
    titleEl.textContent = "Follow Community Leaders";
    var metaEl = document.createElement("div");
    metaEl.className = "archive-item__meta";
    metaEl.textContent = "Follow \u00b7 30 CR earned";
    infoDiv.appendChild(titleEl);
    infoDiv.appendChild(metaEl);
    item.appendChild(infoDiv);

    var checkDiv = document.createElement("div");
    checkDiv.className = "archive-item__check";
    checkDiv.appendChild(
      createSvg("M22 11.08V12a10 10 0 1 1-5.93-9.14", "M22 4L12 14.01 9 11.01"),
    );
    item.appendChild(checkDiv);

    list.appendChild(item);
  }, 750);
}

// Helper: create inline SVG with paths
function createSvg() {
  var ns = "http://www.w3.org/2000/svg";
  var svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("stroke-width", "2");
  svg.setAttribute("stroke-linecap", "round");
  svg.setAttribute("stroke-linejoin", "round");
  for (var i = 0; i < arguments.length; i++) {
    var path = document.createElementNS(ns, "path");
    path.setAttribute("d", arguments[i]);
    svg.appendChild(path);
  }
  return svg;
}
