// Space Viewer Arena — Panel management
(function () {
  window.ratel = window.ratel || {};
  window.ratel.spaceViewer = {
    openPanel: function (panelId) {
      this.closeAllPanels();
      var panel = document.getElementById(panelId);
      if (panel) {
        panel.setAttribute("data-open", "true");
      }
      var portal = document.getElementById("portal");
      var author = document.getElementById("portal-author");
      if (portal) portal.setAttribute("data-dimmed", "true");
      if (author) author.setAttribute("data-dimmed", "true");

      if (panelId === "overview-panel") {
        var btn = document.getElementById("btn-overview");
        if (btn) btn.setAttribute("aria-pressed", "true");
      } else if (panelId === "settings-panel") {
        var btn = document.getElementById("btn-settings");
        if (btn) btn.setAttribute("aria-pressed", "true");
      }
    },

    closeAllPanels: function () {
      var panels = document.querySelectorAll(
        ".overview-panel, .settings-panel"
      );
      panels.forEach(function (p) {
        p.setAttribute("data-open", "false");
      });
      var portal = document.getElementById("portal");
      var author = document.getElementById("portal-author");
      if (portal) portal.setAttribute("data-dimmed", "false");
      if (author) author.setAttribute("data-dimmed", "false");

      var btns = document.querySelectorAll(".hud-btn");
      btns.forEach(function (b) {
        b.setAttribute("aria-pressed", "false");
      });
    },

    togglePanel: function (panelId) {
      var panel = document.getElementById(panelId);
      if (panel && panel.getAttribute("data-open") === "true") {
        this.closeAllPanels();
      } else {
        this.openPanel(panelId);
      }
    },
  };
})();
