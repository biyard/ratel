/* ─────────────────────────────────────────────────────────────
   sub-team / shared.js
   Bootstraps the arena background (stars, shooting stars) and
   common interactions used across every sub-team mockup.
   Call `SubTeam.init()` once DOM is ready.
   ───────────────────────────────────────────────────────────── */

(function(){
  const SubTeam = {};

  // ── 1. Star field (visual noise) ──────────────────────────
  function renderStars(layerId, count) {
    const layer = document.getElementById(layerId);
    if (!layer) return;
    const total = count || 90;
    for (let i = 0; i < total; i++) {
      const s = document.createElement('div');
      s.className = 'star';
      const size = Math.random() < 0.15 ? 2 + Math.random() * 1.5 : 1 + Math.random() * 0.8;
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
    for (let i = 0; i < 2; i++) {
      const ss = document.createElement('div');
      ss.className = 'shooting';
      ss.style.top = (10 + i * 38 + Math.random() * 10) + '%';
      ss.style.left = (Math.random() * 40) + '%';
      ss.style.animationDelay = (-Math.random() * 8 + (i * 3)).toFixed(2) + 's';
      layer.appendChild(ss);
    }
  }

  // ── 2. Toast ──────────────────────────────────────────────
  // <div class="toast" id="toast"><svg>…</svg><span class="toast__text"></span></div>
  let toastTimer = null;
  function toast(message, variant) {
    const el = document.getElementById('toast');
    if (!el) return;
    const text = el.querySelector('.toast__text');
    if (text) text.textContent = message;
    el.classList.remove('toast--success', 'toast--danger');
    if (variant) el.classList.add('toast--' + variant);
    el.setAttribute('data-open', 'true');
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => el.setAttribute('data-open', 'false'), 2500);
  }

  // ── 3. Modal open/close helpers ──────────────────────────
  function openModal(id) {
    const m = document.getElementById(id);
    if (m) m.setAttribute('data-open', 'true');
  }
  function closeModal(id) {
    const m = document.getElementById(id);
    if (m) m.setAttribute('data-open', 'false');
  }
  // Any [data-modal-close] inside a modal closes its parent backdrop
  function bindModalClose() {
    document.querySelectorAll('[data-modal-close]').forEach(btn => {
      btn.addEventListener('click', () => {
        const mb = btn.closest('.modal-backdrop');
        if (mb) mb.setAttribute('data-open', 'false');
      });
    });
    // click outside modal closes
    document.querySelectorAll('.modal-backdrop').forEach(mb => {
      mb.addEventListener('click', (e) => {
        if (e.target === mb) mb.setAttribute('data-open', 'false');
      });
    });
    // ESC closes any open modal
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        document.querySelectorAll('.modal-backdrop[data-open="true"]').forEach(mb => mb.setAttribute('data-open', 'false'));
      }
    });
  }

  // ── 4. Back button ───────────────────────────────────────
  function bindBack() {
    const back = document.getElementById('brand-back');
    if (back) {
      back.addEventListener('click', () => {
        if (history.length > 1) history.back();
        else window.location.href = 'index.html';
      });
    }
    const home = document.getElementById('brand-home');
    if (home) {
      home.addEventListener('click', () => {
        window.location.href = 'index.html';
      });
    }
  }

  // ── 5. Init ──────────────────────────────────────────────
  function init(opts) {
    opts = opts || {};
    renderStars('stars', opts.stars || 90);
    bindModalClose();
    bindBack();
  }

  SubTeam.init = init;
  SubTeam.toast = toast;
  SubTeam.openModal = openModal;
  SubTeam.closeModal = closeModal;
  window.SubTeam = SubTeam;

  // Auto-init on DOM ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => SubTeam.init());
  } else {
    SubTeam.init();
  }
})();
