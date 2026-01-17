pub fn build_report_html_document(fragment: &str) -> String {
    let d3_src = "https://cdn.jsdelivr.net/npm/d3@7/dist/d3.min.js";

    format!(
        r#"<!doctype html>
<html lang="ko">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title></title>

  <style>
    @font-face {{
      font-family: "NotoSansKR";
      src: url("https://metadata.ratel.foundation/fonts/NotoSansKR-Regular.ttf") format("truetype");
      font-weight: 400;
      font-style: normal;
    }}
    @font-face {{
      font-family: "NotoSansKR";
      src: url("https://metadata.ratel.foundation/fonts/NotoSansKR-Bold.ttf") format("truetype");
      font-weight: 700;
      font-style: normal;
    }}

    :root {{ color-scheme: dark; }}

    html, body {{
      margin: 0;
      padding: 0;
      background: #0b0f14;
      color: #e5e7eb;
      font-family: "NotoSansKR", system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif;
    }}

    @media print {{
      :root {{ color-scheme: light; }}
      html, body {{
        background: #ffffff !important;
        color: #111827 !important;
      }}
      .page {{
        background: #ffffff !important;
      }}
      p, td, th {{
        color: #111827 !important;
      }}
      table, th, td {{
        border-color: rgba(0,0,0,0.85) !important;
      }}
    }}

    .page {{ padding: 24px 20px; }}

    table {{
      width: 100%;
      border-collapse: collapse;
      font-size: 12px;
      margin-top: 8px;
      border: 2px solid rgba(17,24,39,0.85);
    }}

    th, td {{
      border: 2px solid rgba(17,24,39,0.85);
      padding: 14px 12px;
      text-align: center;
      vertical-align: middle;
      background: transparent;
      color: rgba(17,24,39,0.92);
      font-size: 12px;
      line-height: 1.55;
    }}

    th {{
      font-weight: 700;
    }}

    .lda-topic {{
      font-weight: 700;
      white-space: nowrap;
    }}

    .lda-keywords {{
      font-weight: 400;
      text-align: center;
      word-break: keep-all;
    }}

    .tfidf-wrap {{
      margin-top: 14px;
      width: 100%;
    }}

    .tfidf-svg {{
      width: 100%;
      display: block;
    }}

    .network-wrap {{
      margin-top: 18px;
      padding: 0;
      border: none;
      border-radius: 0;
      background: transparent;
    }}
  </style>

  <script src="{d3_src}"></script>
</head>

<body>
  <div class="page">
    <div id="content-root">{fragment}</div>
    <div id="__render_marker" style="display:none;"></div>
  </div>

  <script>
    window.__REPORT_BOOTED__ = true;
    window.__REPORT_RENDER_DONE__ = false;
    window.__REPORT_STAGE__ = "boot";
    window.__REPORT_ERROR__ = null;

    function setError(e) {{
      try {{
        window.__REPORT_ERROR__ =
          (e && e.stack) ? String(e.stack) :
          (typeof e === "string") ? e :
          JSON.stringify(e);
      }} catch (_) {{
        window.__REPORT_ERROR__ = String(e);
      }}
    }}

    window.onerror = function (msg, src, line, col, err) {{
      window.__REPORT_STAGE__ = "window.onerror";
      setError((err && err.stack) ? err.stack : (String(msg) + " @" + line + ":" + col));
      window.__REPORT_RENDER_DONE__ = true;
      return false;
    }};

    window.onunhandledrejection = function (evt) {{
      window.__REPORT_STAGE__ = "unhandledrejection";
      setError(evt && evt.reason ? evt.reason : evt);
      window.__REPORT_RENDER_DONE__ = true;
    }};

    function decodePayload(b64) {{
      const bin = atob(b64);
      const bytes = new Uint8Array(bin.length);
      for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
      const jsonStr = new TextDecoder("utf-8").decode(bytes);
      return JSON.parse(jsonStr);
    }}

    function renderLDA(host, payload) {{
      host.innerHTML = "";

      const table = document.createElement("table");

      const colgroup = document.createElement("colgroup");
      const c1 = document.createElement("col");
      const c2 = document.createElement("col");
      c1.style.width = "28%";
      c2.style.width = "72%";
      colgroup.appendChild(c1);
      colgroup.appendChild(c2);
      table.appendChild(colgroup);

      const thead = document.createElement("thead");
      const trh = document.createElement("tr");

      const th1 = document.createElement("th");
      th1.textContent = "주제";

      const th2 = document.createElement("th");
      th2.textContent = "키워드";

      trh.appendChild(th1);
      trh.appendChild(th2);
      thead.appendChild(trh);
      table.appendChild(thead);

      const tbody = document.createElement("tbody");
      const rows = payload.ldaTopics || payload.lda_topics || [];
      const map = new Map();

      for (const row of rows) {{
        const t = String(row.topic || "").trim();
        const k = String(row.keyword || "").trim();
        if (!t || !k) continue;
        if (!map.has(t)) map.set(t, []);
        map.get(t).push(k);
      }}

      for (const entry of map.entries()) {{
        const topic = entry[0];
        const keywords = entry[1];

        const tr = document.createElement("tr");

        const td1 = document.createElement("td");
        td1.className = "lda-topic";
        td1.textContent = topic;

        const td2 = document.createElement("td");
        td2.className = "lda-keywords";
        td2.textContent = keywords.join(", ");

        tr.appendChild(td1);
        tr.appendChild(td2);
        tbody.appendChild(tr);
      }}

      table.appendChild(tbody);
      host.appendChild(table);
    }}

    function sleep(ms) {{ return new Promise(r => setTimeout(r, ms)); }}

    async function waitFor(condFn, timeoutMs) {{
      const start = Date.now();
      while (Date.now() - start < timeoutMs) {{
        try {{
          if (condFn()) return true;
        }} catch (_) {{}}
        await sleep(50);
      }}
      return false;
    }}

    async function renderTFIDF(host, payload) {{
      window.__REPORT_STAGE__ = "tfidf:wait_d3";
      const ok = await waitFor(() => typeof window.d3 !== "undefined", 8000);
      if (!ok) throw new Error("d3 not loaded (check CDN/network).");

      window.__REPORT_STAGE__ = "tfidf:render";
      host.innerHTML = "";

      const wrap = document.createElement("div");
      wrap.className = "tfidf-wrap";
      host.appendChild(wrap);

      const rowsRaw = Array.isArray(payload.tf_idf) ? payload.tf_idf : [];
      const rows = rowsRaw
        .map(d => ({{ key: String(d.keyword || ""), val: Number(d.tf_idf || 0) }}))
        .filter(d => d.key.length > 0);

      if (rows.length === 0) return;

      const rect = wrap.getBoundingClientRect();
      const width = Math.max(360, Math.floor(rect.width || 900));

      let maxVal = 0;
      let maxLen = 0;
      for (const d of rows) {{
        maxVal = Math.max(maxVal, d.val);
        maxLen = Math.max(maxLen, d.key.length);
      }}
      if (maxVal <= 0) maxVal = 1;

      function niceCeil(v, step) {{
        const s = Number(step || 1);
        return Math.ceil((Number(v) || 0) / s) * s;
      }}

      const xMax = Math.max(1, niceCeil(maxVal, 0.5));
      const tickStep = 0.5;

      const margin = {{
        top: 10,
        right: 52,
        bottom: 34,
        left: Math.min(210, Math.max(110, maxLen * 16 + 38)),
      }};

      const rowH = 34;
      const height = margin.top + margin.bottom + rows.length * rowH;

      const svgEl = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svgEl.setAttribute("class", "tfidf-svg");
      svgEl.setAttribute("width", String(width));
      svgEl.setAttribute("height", String(height));
      svgEl.setAttribute("viewBox", "0 0 " + width + " " + height);
      wrap.appendChild(svgEl);

      const s = window.d3.select(svgEl);

      const x = window.d3.scaleLinear()
        .domain([0, xMax])
        .range([margin.left, width - margin.right]);

      const y = window.d3.scaleBand()
        .domain(rows.map(d => d.key))
        .range([margin.top, height - margin.bottom])
        .padding(0.20);

      const ticks = [];
      for (let t = 0; t <= xMax + 1e-9; t += tickStep) ticks.push(Number(t.toFixed(2)));

      const xAxis = window.d3.axisBottom(x)
        .tickValues(ticks)
        .tickFormat(d => Number(d).toFixed(1))
        .tickSize(0)
        .tickSizeOuter(0);

      const yAxis = window.d3.axisLeft(y)
        .tickSize(0)
        .tickSizeOuter(0)
        .tickPadding(18);

      const barX0 = margin.left + 18;

      s.append("g")
        .selectAll("rect")
        .data(rows)
        .join("rect")
        .attr("x", barX0)
        .attr("y", d => y(d.key))
        .attr("height", y.bandwidth())
        .attr("width", d => Math.max(0, x(d.val) - barX0))
        .attr("fill", "rgba(23, 107, 135, 0.92)");

      s.append("g")
        .attr("transform", "translate(0," + (height - margin.bottom) + ")")
        .call(xAxis)
        .call(g => g.selectAll(".tick line").remove())
        .call(g => g.selectAll(".tick text").style("font-size", "15px").style("fill", "rgba(17,24,39,0.75)"))
        .call(g => g.select(".domain").remove());

      s.append("g")
        .attr("transform", "translate(" + margin.left + ",0)")
        .call(yAxis)
        .call(g => g.select(".domain").remove())
        .call(g => g.selectAll(".tick line").remove())
        .call(g => g.selectAll(".tick text").style("font-size", "12px").style("fill", "rgba(17,24,39,0.75)"));

      s.append("g")
        .selectAll("text.tfidf-value")
        .data(rows)
        .join("text")
        .attr("class", "tfidf-value")
        .attr("x", d => x(d.val) + 10)
        .attr("y", d => (y(d.key) ?? 0) + y.bandwidth() / 2)
        .attr("dy", "0.32em")
        .style("font-size", "12px")
        .style("font-weight", "700")
        .style("fill", "rgba(17,24,39,0.90)")
        .text(d => d.val.toFixed(2));
    }}

    async function renderNetwork(host, payload) {{
      window.__REPORT_STAGE__ = "network:wait_d3";
      const ok = await waitFor(() => typeof window.d3 !== "undefined", 8000);
      if (!ok) throw new Error("d3 not loaded (check CDN/network).");

      window.__REPORT_STAGE__ = "network:render";
      host.innerHTML = "";

      const wrap = document.createElement("div");
      wrap.className = "network-wrap";

      const canvas = document.createElement("canvas");
      canvas.style.width = "100%";
      canvas.style.height = "520px";
      canvas.style.display = "block";
      wrap.appendChild(canvas);
      host.appendChild(wrap);

      const rect = wrap.getBoundingClientRect();
      const width = Math.max(1, Math.floor(rect.width || 900));
      const height = 520;

      const dpr = Math.max(1, Math.min(2, window.devicePixelRatio || 1));
      canvas.width = Math.floor(width * dpr);
      canvas.height = Math.floor(height * dpr);

      const ctx = canvas.getContext("2d");
      if (!ctx) throw new Error("canvas ctx is null");
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

      const data = payload.network || {{ nodes: [], edges: [] }};
      const nodesRaw = Array.isArray(data.nodes) ? data.nodes : [];
      const edgesRaw = Array.isArray(data.edges) ? data.edges : [];

      const nodes = nodesRaw
        .map(n => ({{
          id: String(n?.node ?? "").trim(),
          degree: Number(n?.degree_centrality ?? 0),
          betweenness: Number(n?.betweenness_centrality ?? 0),
          rank: 9999,
          x: 0,
          y: 0,
          vx: 0,
          vy: 0,
        }}))
        .filter(n => n.id.length > 0);

      const nodeMap = new Map();
      for (const n of nodes) nodeMap.set(n.id, n);

      const links = edgesRaw
        .map(e => ({{
          source: String(e?.source ?? "").trim(),
          target: String(e?.target ?? "").trim(),
          weight: Number(e?.weight ?? 0),
        }}))
        .filter(l => l.source && l.target && l.source !== l.target && nodeMap.has(l.source) && nodeMap.has(l.target));

      let maxWeight = 0;
      for (const l of links) maxWeight = Math.max(maxWeight, l.weight || 0);

      const scoreOf = (n) => Math.max(n.degree || 0, n.betweenness || 0);
      const sorted = [...nodes].sort((a, b) => scoreOf(b) - scoreOf(a));
      for (let i = 0; i < sorted.length; i++) sorted[i].rank = i;

      function getNodeRadius(n) {{
        const rnk = n.rank ?? 9999;
        if (rnk === 0) return 92;
        if (rnk <= 2) return 72;
        if (rnk <= 5) return 58;
        if (rnk <= 10) return 46;
        if (rnk <= 20) return 36;
        return 30;
      }}

      function getLinkAlpha(w) {{
        const ww = Number(w ?? 0);
        if (maxWeight <= 0) return 0.10;
        const rr = ww / maxWeight;
        return 0.04 + rr * 0.14;
      }}

      function getLinkWidth(w) {{
        const ww = Number(w ?? 0);
        if (maxWeight <= 0) return 0.8;
        return 0.6 + (ww / maxWeight) * 1.1;
      }}

      const cx = width / 2;
      const cy = height / 2;

      for (let i = 0; i < nodes.length; i++) {{
        const a = (i / Math.max(1, nodes.length)) * Math.PI * 2;
        nodes[i].x = cx + Math.cos(a) * 10;
        nodes[i].y = cy + Math.sin(a) * 10;
        nodes[i].vx = 0;
        nodes[i].vy = 0;
      }}

      const chargeStrength = Math.max(-900, Math.min(-260, -420 - nodes.length * 9));

      const linkForce = window.d3.forceLink(links)
        .id(d => d.id)
        .distance(l => {{
          const w = Number(l?.weight ?? 0);
          const rr = maxWeight > 0 ? w / maxWeight : 0;
          return 190 - rr * 90;
        }})
        .strength(l => {{
          const w = Number(l?.weight ?? 0);
          const rr = maxWeight > 0 ? w / maxWeight : 0;
          return 0.08 + rr * 0.28;
        }});

      const sim = window.d3.forceSimulation(nodes)
        .force("link", linkForce)
        .force("charge", window.d3.forceManyBody().strength(chargeStrength))
        .force("center", window.d3.forceCenter(0, 0))
        .force("collide", window.d3.forceCollide().radius(n => getNodeRadius(n) + 18).iterations(2));

      function computeBounds() {{
        let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
        for (const n of nodes) {{
          const x = Number(n.x), y = Number(n.y);
          if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
          const r = getNodeRadius(n);
          minX = Math.min(minX, x - r);
          maxX = Math.max(maxX, x + r);
          minY = Math.min(minY, y - r);
          maxY = Math.max(maxY, y + r);
        }}
        if (!Number.isFinite(minX)) return null;
        return {{ minX, maxX, minY, maxY }};
      }}

      function fitTransform() {{
        const b = computeBounds();
        if (!b) return {{ k: 1, tx: cx, ty: cy }};

        const pad = 54;
        const bw = Math.max(1, b.maxX - b.minX);
        const bh = Math.max(1, b.maxY - b.minY);

        const kx = (width - pad * 2) / bw;
        const ky = (height - pad * 2) / bh;
        const k = Math.min(2.3, Math.max(0.2, Math.min(kx, ky)));

        const mx = (b.minX + b.maxX) / 2;
        const my = (b.minY + b.maxY) / 2;

        const tx = cx - mx * k;
        const ty = cy - my * k;

        return {{ k, tx, ty }};
      }}

      function draw() {{
        ctx.clearRect(0, 0, width, height);

        const tf = fitTransform();
        const k = tf.k, tx = tf.tx, ty = tf.ty;

        const mapX = (x) => x * k + tx;
        const mapY = (y) => y * k + ty;

        for (const l of links) {{
          const s = typeof l.source === "string" ? nodeMap.get(l.source) : l.source;
          const t = typeof l.target === "string" ? nodeMap.get(l.target) : l.target;
          if (!s || !t) continue;

          const alpha = getLinkAlpha(l.weight);
          const w = getLinkWidth(l.weight);

          ctx.beginPath();
          ctx.moveTo(mapX(s.x), mapY(s.y));
          ctx.lineTo(mapX(t.x), mapY(t.y));
          ctx.lineWidth = Math.max(0.8, w / Math.max(0.9, k));
          ctx.strokeStyle = "rgba(120,120,120," + alpha.toFixed(3) + ")";
          ctx.stroke();
        }}

        for (const n of nodes) {{
          const x = mapX(n.x);
          const y = mapY(n.y);
          const rr = getNodeRadius(n) * k;

          ctx.beginPath();
          ctx.arc(x, y, rr, 0, Math.PI * 2);
          ctx.fillStyle = "rgba(171, 215, 231, 0.85)";
          ctx.fill();

          ctx.lineWidth = Math.max(2, (2.5 / Math.max(0.9, k)));
          ctx.strokeStyle = "rgba(55, 65, 81, 0.70)";
          ctx.stroke();

          const fontSize = Math.max(12, Math.min(22, getNodeRadius(n) * 0.42)) * k;
          ctx.font = "700 " + Math.max(12, fontSize).toFixed(2) + "px sans-serif";
          ctx.fillStyle = "rgba(17, 24, 39, 0.92)";
          ctx.textAlign = "center";
          ctx.textBaseline = "middle";
          ctx.fillText(n.id, x, y);
        }}
      }}

      sim.on("tick", () => {{ draw(); }});
      sim.alpha(1);
      sim.restart();
      for (let i = 0; i < 260; i++) sim.tick();
      draw();
      sim.stop();
    }}

    async function main() {{
      window.__REPORT_STAGE__ = "main:start";

      const blocks = Array.from(document.querySelectorAll("div[data-analyze][data-payload]"));
      for (const el of blocks) {{
        const kind = el.getAttribute("data-analyze");
        const b64 = el.getAttribute("data-payload");
        if (!kind || !b64) continue;

        const payload = decodePayload(b64);

        if (kind === "lda") renderLDA(el, payload);
        if (kind === "tfidf") await renderTFIDF(el, payload);
        if (kind === "network") await renderNetwork(el, payload);

        el.removeAttribute("data-analyze");
        el.removeAttribute("data-payload");
      }}

      window.__REPORT_STAGE__ = "fonts:wait";
      if (document.fonts && document.fonts.ready) {{
        try {{ await document.fonts.ready; }} catch (_) {{}}
      }}

      await new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)));
      window.__REPORT_STAGE__ = "main:done";
    }}

    async function run() {{
      try {{
        window.__REPORT_STAGE__ = "run:enter";
        await main();
      }} catch (e) {{
        window.__REPORT_STAGE__ = "run:catch";
        setError(e);
      }} finally {{
        window.__REPORT_STAGE__ = "run:finally";
        window.__REPORT_RENDER_DONE__ = true;
        const m = document.getElementById("__render_marker");
        if (m) m.textContent = "done";
      }}
    }}

    if (document.readyState === "loading") {{
      document.addEventListener("DOMContentLoaded", run);
    }} else {{
      run();
    }}

    setTimeout(() => {{
      if (window.__REPORT_RENDER_DONE__ !== true) {{
        window.__REPORT_STAGE__ = "watchdog";
        setError(window.__REPORT_ERROR__ || "watchdog timeout");
        window.__REPORT_RENDER_DONE__ = true;
      }}
    }}, 20000);
  </script>
</body>
</html>
"#,
        d3_src = d3_src,
        fragment = fragment
    )
}
