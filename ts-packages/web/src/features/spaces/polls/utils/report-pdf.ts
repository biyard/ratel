const waitFor = async (fn: () => boolean, timeoutMs: number) => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (fn()) return true;
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  return false;
};

export const buildReportPdfBlob = async (
  htmlDocument: string,
): Promise<Blob> => {
  const iframe = document.createElement('iframe');
  iframe.style.position = 'fixed';
  iframe.style.left = '-9999px';
  iframe.style.top = '0';
  iframe.style.width = '1280px';
  iframe.style.height = '720px';
  iframe.style.border = '0';
  iframe.srcdoc = htmlDocument;
  document.body.appendChild(iframe);

  const ready = await waitFor(
    () => iframe.contentDocument?.readyState === 'complete',
    10_000,
  );
  if (!ready) {
    iframe.remove();
    throw new Error('failed to load report html');
  }

  const rendered = await waitFor(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    () => (iframe.contentWindow as any)?.__REPORT_RENDER_DONE__ === true,
    60_000,
  );
  if (!rendered) {
    iframe.remove();
    throw new Error('report render timeout');
  }

  const doc = iframe.contentDocument;
  if (!doc) {
    iframe.remove();
    throw new Error('report document not found');
  }

  const target =
    (doc.querySelector('#content-root') as HTMLElement | null) ??
    (doc.body as HTMLElement | null);

  if (!target) {
    iframe.remove();
    throw new Error('report root not found');
  }

  const marginMm = [6, 10, 10, 10] as const;
  const pxPerMm = 96 / 25.4;
  const pageInnerHeightPx = (297 - marginMm[0] - marginMm[2]) * pxPerMm;

  const forceCss = `
    :root{
      color-scheme: light !important;
      --color-panel-table-header: #000000 !important;
      --color-text-primary: #000000 !important;
      --color-text-secondary: #000000 !important;
      --color-input-box-border: #000000 !important;
      --input-box-border: #000000 !important;
      --border: #000000 !important;
    }

    html, body{
      background:#ffffff !important;
      color:#000000 !important;
      -webkit-print-color-adjust: exact !important;
      print-color-adjust: exact !important;
    }

    #content-root{
      font-family: "NotoSansKR", system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif !important;
      -webkit-font-smoothing: antialiased !important;
      text-rendering: geometricPrecision !important;
      color:#000000 !important;
      -webkit-text-fill-color:#000000 !important;
    }

    #content-root *{
      box-sizing:border-box !important;
      color:#000000 !important;
      -webkit-text-fill-color:#000000 !important;
      caret-color:#000000 !important;
      opacity:1 !important;
      filter:none !important;
      text-shadow:none !important;
      mix-blend-mode:normal !important;
      -webkit-text-stroke:0 !important;
    }

    #content-root del,
    #content-root s,
    #content-root strike{
      text-decoration: none !important;
      position: relative !important;
      background-image: linear-gradient(#000000, #000000) !important;
      background-repeat: repeat-x !important;
      background-size: 100% 1.5px !important;
      background-position: 0 60% !important;
      -webkit-box-decoration-break: clone !important;
      box-decoration-break: clone !important;
      padding: 0 0.02em !important;
    }

    #content-root h1{
      font-size: 38px !important;
      font-weight: 400 !important;
      line-height: 1.16 !important;
      margin: 0 0 12px 0 !important;
    }

    #content-root h2{
      font-size: 30px !important;
      font-weight: 400 !important;
      line-height: 1.18 !important;
      margin: 0 0 12px 0 !important;
    }

    #content-root h3{
      font-size: 24px !important;
      font-weight: 400 !important;
      line-height: 1.2 !important;
      margin: 0 0 10px 0 !important;
    }

    #content-root h1 *,
    #content-root h2 *,
    #content-root h3 *{
      font-weight: 400 !important;
    }

    #content-root h1 strong,
    #content-root h2 strong,
    #content-root h3 strong,
    #content-root h1 b,
    #content-root h2 b,
    #content-root h3 b{
      font-weight: 700 !important;
    }

    #content-root p{
      font-size: 16px !important;
      margin-top: 0 !important;
      margin-bottom: 10px !important;
      line-height: 1.5 !important;
    }

    #content-root li{
      font-size: 16px !important;
      line-height: 1.5 !important;
    }

    #content-root svg text{
      fill:#000000 !important;
    }

    #content-root ul,
    #content-root ol{
      margin: 0 0 10px 0 !important;
      padding-left: 0 !important;
      list-style: none !important;
    }

    #content-root li[data-pdf-li="1"]{
      position: relative !important;
      padding-left: 28px !important;
      margin: 0 0 6px 0 !important;
    }

    #content-root .__pdf_marker{
      position: absolute !important;
      left: 0 !important;
      top: 0 !important;
      width: 24px !important;
      height: 1.5em !important;
      display: flex !important;
      align-items: center !important;
      justify-content: center !important;
      font-weight: 400 !important;
      font-size: 16px !important;
      line-height: 1.5 !important;
      color: #000000 !important;
      -webkit-text-fill-color:#000000 !important;
      white-space: nowrap !important;
      overflow: visible !important;
    }

    #content-root table{
      width:100% !important;
      border-collapse: collapse !important;
      border: 1.5px solid #000000 !important;
      background:#ffffff !important;
      table-layout: fixed !important;
      margin-top: 10px !important;
      position: relative !important;
      z-index: 0 !important;
    }

    #content-root * + table{
      margin-top: 10px !important;
    }

    #content-root table[data-pdf-two-col="1"] th:nth-child(1),
    #content-root table[data-pdf-two-col="1"] td:nth-child(1){
      width: 14% !important;
      white-space: nowrap !important;
    }

    #content-root table[data-pdf-two-col="1"] th:nth-child(2),
    #content-root table[data-pdf-two-col="1"] td:nth-child(2){
      width: 86% !important;
    }

    #content-root th, #content-root td{
      border: 1.5px solid #000000 !important;
      border-color:#000000 !important;
      background:#ffffff !important;
      padding: 0 !important;
      vertical-align: middle !important;
      height: 56px !important;
    }

    #content-root .__pdf_cell{
      width: 100% !important;
      height: 100% !important;
      display: grid !important;
      place-items: center !important;
      padding: 10px 14px 12px 14px !important;
      font-size: 14.5px !important;
      line-height: 1.35 !important;
      text-align: center !important;
      position: relative !important;
      z-index: 1 !important;
    }

    #content-root tbody td .__pdf_cell{
      white-space: normal !important;
      word-break: keep-all !important;
      overflow-wrap: anywhere !important;
      text-align: center !important;
    }

    #content-root thead th .__pdf_cell{
      font-weight: 900 !important;
    }

    [data-pdf-atomic="1"]{
      break-inside: avoid !important;
      page-break-inside: avoid !important;
    }

    #content-root .min-h-screen,
    #content-root .h-screen{
      min-height: auto !important;
      height: auto !important;
    }

    #content-root [style*="100vh"]{
      min-height: auto !important;
      height: auto !important;
    }

    #content-root section,
    #content-root article,
    #content-root .card,
    #content-root .panel,
    #content-root .report-section{
      margin-top: 0 !important;
      padding-top: 0 !important;
    }
  `;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const html2pdf = (await import('html2pdf.js')).default as any;

  await new Promise((r) =>
    requestAnimationFrame(() => requestAnimationFrame(r)),
  );

  const countColumns = (row: HTMLTableRowElement | null) => {
    if (!row) return 0;
    const cells = Array.from(row.cells);
    let n = 0;
    for (const c of cells) {
      const colspan = Number((c as HTMLTableCellElement).colSpan || 1);
      n += Number.isFinite(colspan) && colspan > 0 ? colspan : 1;
    }
    return n;
  };

  const markTwoColTables = (root: HTMLElement) => {
    const tables = Array.from(
      root.querySelectorAll('table'),
    ) as HTMLTableElement[];
    for (const t of tables) {
      const headRow =
        (t.tHead?.rows?.[0] as HTMLTableRowElement | undefined) ?? null;
      const bodyRow =
        (t.tBodies?.[0]?.rows?.[0] as HTMLTableRowElement | undefined) ?? null;
      const cols = Math.max(countColumns(headRow), countColumns(bodyRow));
      if (cols === 2) t.setAttribute('data-pdf-two-col', '1');
      else t.removeAttribute('data-pdf-two-col');
    }
  };

  const computeOlPrefix = (li: HTMLLIElement) => {
    const parts: number[] = [];
    let curLi: HTMLLIElement | null = li;

    while (curLi) {
      const parentOl = curLi.parentElement?.closest(
        'ol',
      ) as HTMLOListElement | null;
      if (!parentOl) break;

      const siblings = Array.from(parentOl.children).filter(
        (n) => (n as HTMLElement).tagName === 'LI',
      ) as HTMLLIElement[];
      const idx = Math.max(0, siblings.indexOf(curLi)) + 1;
      parts.unshift(idx);

      curLi = parentOl.closest('li') as HTMLLIElement | null;
    }

    return parts.length ? `${parts.join('.')}.` : '1.';
  };

  const materializeListMarkers = (root: HTMLElement) => {
    const lists = Array.from(root.querySelectorAll('ul, ol')) as (
      | HTMLUListElement
      | HTMLOListElement
    )[];
    for (const list of lists) {
      const isOl = list.tagName === 'OL';
      const lis = Array.from(
        list.querySelectorAll(':scope > li'),
      ) as HTMLLIElement[];

      for (const li of lis) {
        li.setAttribute('data-pdf-li', '1');

        const already = li.querySelector(
          ':scope > .__pdf_marker',
        ) as HTMLSpanElement | null;
        if (already) continue;

        const marker = root.ownerDocument.createElement('span');
        marker.className = '__pdf_marker';
        marker.textContent = isOl ? computeOlPrefix(li) : '•';

        li.insertBefore(marker, li.firstChild);
      }
    }
  };

  const markAtomicBlocks = (root: HTMLElement) => {
    const atomicRoots = new Set<HTMLElement>();

    const candidates = Array.from(
      root.querySelectorAll(
        [
          '[data-pdf-keep]',
          '.recharts-wrapper',
          '.recharts-responsive-container',
          '.chart',
          '.graph',
          '[class*="chart"]',
          '[class*="graph"]',
          'canvas',
          'svg',
          'figure',
          'img',
        ].join(','),
      ),
    ) as HTMLElement[];

    for (const el of candidates) {
      const container =
        (el.closest('.recharts-responsive-container') as HTMLElement | null) ??
        (el.closest('.recharts-wrapper') as HTMLElement | null) ??
        (el.closest('[data-pdf-keep]') as HTMLElement | null) ??
        (el.closest(
          '.chart, .graph, [class*="chart"], [class*="graph"]',
        ) as HTMLElement | null) ??
        (el.closest('figure') as HTMLElement | null) ??
        el;

      atomicRoots.add(container);
    }

    for (const a of atomicRoots) {
      a.setAttribute('data-pdf-atomic', '1');
      const s = a.style as CSSStyleDeclaration;
      s.breakInside = 'avoid';
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (s as any).pageBreakInside = 'avoid';
    }
  };

  const insertPageBreakSpacers = (root: HTMLElement) => {
    const clearOld = Array.from(root.querySelectorAll('[data-pdf-spacer="1"]'));
    for (const n of clearOld) n.remove();

    const atomicBlocks = Array.from(
      root.querySelectorAll('[data-pdf-atomic="1"]'),
    ) as HTMLElement[];

    const atomicSet = new Set(atomicBlocks);

    const baseSelectors = [
      'h1',
      'h2',
      'h3',
      'p',
      'ul',
      'ol',
      'table',
      'pre',
      'blockquote',
      'img',
      'figure',
    ].join(',');

    const blocks = [
      ...atomicBlocks,
      ...(
        Array.from(root.querySelectorAll(baseSelectors)) as HTMLElement[]
      ).filter((el) => {
        let p: HTMLElement | null = el;
        while (p && p !== root) {
          if (atomicSet.has(p)) return false;
          p = p.parentElement;
        }
        return true;
      }),
    ].filter((el) => {
      const r = el.getBoundingClientRect();
      return r.width > 0 && r.height > 0;
    });

    const maxIter = 10;
    for (let iter = 0; iter < maxIter; iter += 1) {
      let changed = false;
      const rootRect = root.getBoundingClientRect();

      for (const el of blocks) {
        if (!el.isConnected) continue;

        const r = el.getBoundingClientRect();
        const y = r.top - rootRect.top;
        const h = r.height;

        if (h >= pageInnerHeightPx * 0.98) continue;

        const mod =
          ((y % pageInnerHeightPx) + pageInnerHeightPx) % pageInnerHeightPx;
        const overflow = mod + h - pageInnerHeightPx;

        if (overflow > 2) {
          const spacerH = pageInnerHeightPx - mod + 2;
          const spacer = root.ownerDocument.createElement('div');
          spacer.setAttribute('data-pdf-spacer', '1');
          spacer.style.height = `${spacerH}px`;
          spacer.style.width = '1px';
          spacer.style.pointerEvents = 'none';
          spacer.style.background = 'transparent';
          el.parentElement?.insertBefore(spacer, el);
          changed = true;
        }
      }

      if (!changed) break;
    }
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const worker: any = html2pdf()
    .from(target)
    .set({
      margin: [...marginMm],
      filename: 'report.pdf',
      html2canvas: {
        scale: 2,
        useCORS: true,
        backgroundColor: '#ffffff',
        onclone: (clonedDoc: Document) => {
          const st = clonedDoc.createElement('style');
          st.setAttribute('data-force-black', '1');
          st.textContent = forceCss;
          clonedDoc.head.appendChild(st);

          const root =
            (clonedDoc.querySelector('#content-root') as HTMLElement | null) ??
            (clonedDoc.body as HTMLElement | null);

          if (!root) return;

          markTwoColTables(root);

          const cells = Array.from(
            clonedDoc.querySelectorAll(
              '#content-root table th, #content-root table td',
            ),
          );

          for (const cell of cells) {
            const el = cell as HTMLElement;
            if (el.querySelector(':scope > .__pdf_cell')) continue;

            const wrap = clonedDoc.createElement('div');
            wrap.className = '__pdf_cell';

            while (el.firstChild) {
              wrap.appendChild(el.firstChild);
            }
            el.appendChild(wrap);
          }

          materializeListMarkers(root);
          markAtomicBlocks(root);
          insertPageBreakSpacers(root);
        },
      },
      jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' },
    });

  const blob = await worker.outputPdf('blob');
  iframe.remove();
  return blob;
};

export const uploadReportPdf = async (presignedUrl: string, blob: Blob) => {
  const res = await fetch(presignedUrl, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/pdf',
    },
    body: blob,
  });
  if (!res.ok) {
    throw new Error('failed to upload pdf');
  }
};
