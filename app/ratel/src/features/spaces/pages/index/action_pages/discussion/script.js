(function () {
  function init() {
    var sheet = document.getElementById("discussion-comments-sheet");
    if (!sheet || sheet.dataset.bound) return;
    sheet.dataset.bound = "true";

    var handle = sheet.querySelector(".sheet-handle");
    if (!handle) return;

    var expanded = false;
    var startY = 0;
    var startTranslate = 0;
    var dragging = false;
    var collapsedOffset = 0;

    function getCollapsedOffset() {
      return sheet.offsetHeight - 64;
    }

    function toggle() {
      expanded = !expanded;
      sheet.classList.toggle("expanded", expanded);
    }

    handle.addEventListener("click", function () {
      if (dragging) return;
      toggle();
    });

    // Touch drag
    handle.addEventListener(
      "touchstart",
      function (e) {
        startY = e.touches[0].clientY;
        collapsedOffset = getCollapsedOffset();
        startTranslate = expanded ? 0 : collapsedOffset;
        dragging = false;
        sheet.classList.add("dragging");
      },
      { passive: true }
    );

    handle.addEventListener(
      "touchmove",
      function (e) {
        var dy = e.touches[0].clientY - startY;
        if (Math.abs(dy) < 5 && !dragging) return;
        dragging = true;
        e.preventDefault();
        var next = Math.max(0, Math.min(collapsedOffset, startTranslate + dy));
        sheet.style.transform = "translateY(" + next + "px)";
      },
      { passive: false }
    );

    handle.addEventListener("touchend", function (e) {
      sheet.classList.remove("dragging");
      if (!dragging) return;

      var dy = e.changedTouches[0].clientY - startY;

      if (expanded) {
        if (dy > 60) expanded = false;
      } else {
        if (dy < -60) expanded = true;
      }

      sheet.style.transform = "";
      sheet.classList.toggle("expanded", expanded);
      setTimeout(function () {
        dragging = false;
      }, 50);
    });

    // Mouse drag (desktop testing)
    handle.addEventListener("mousedown", function (e) {
      startY = e.clientY;
      collapsedOffset = getCollapsedOffset();
      startTranslate = expanded ? 0 : collapsedOffset;
      dragging = false;
      sheet.classList.add("dragging");

      function onMouseMove(e) {
        var dy = e.clientY - startY;
        if (Math.abs(dy) < 5 && !dragging) return;
        dragging = true;
        var next = Math.max(0, Math.min(collapsedOffset, startTranslate + dy));
        sheet.style.transform = "translateY(" + next + "px)";
      }

      function onMouseUp(e) {
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
        sheet.classList.remove("dragging");
        if (!dragging) return;

        var dy = e.clientY - startY;
        if (expanded) {
          if (dy > 60) expanded = false;
        } else {
          if (dy < -60) expanded = true;
        }

        sheet.style.transform = "";
        sheet.classList.toggle("expanded", expanded);
        dim.classList.toggle("active", expanded);
        setTimeout(function () {
          dragging = false;
        }, 50);
      }

      document.addEventListener("mousemove", onMouseMove);
      document.addEventListener("mouseup", onMouseUp);
    });
  }

  init();
  new MutationObserver(function () {
    init();
  }).observe(document.body, { childList: true, subtree: true });
})();
