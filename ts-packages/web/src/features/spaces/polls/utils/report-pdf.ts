const waitFor = async (fn: () => boolean, timeoutMs: number) => {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (fn()) return true;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    await new Promise((resolve) => setTimeout(resolve, 50) as any);
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
      margin: 0 !important;
      padding: 0 !important;
    }

    #content-root .__pdf_analyze_title{
      display:block !important;
      text-align:center !important;
      margin: 0 0 8px 0 !important;
      padding: 0 !important;
      font-size: 18px !important;
      line-height: 1.25 !important;
      font-weight: 400 !important;
    }

    #content-root .__pdf_analyze_title + div:empty{
      height: 0 !important;
      margin: 0 !important;
      padding: 0 !important;
    }

    #content-root .__pdf_analyze_caption{
      display:block !important;
      text-align:center !important;
      margin: 10px 0 0 0 !important;
      padding: 0 !important;
      font-size: 14px !important;
      line-height: 1.25 !important;
      font-weight: 400 !important;
    }

    #content-root .lda-footnote,
    #content-root .tfidf-footnote,
    #content-root .network-footnote,
    #content-root .table-footnote,
    #content-root .image-footnote{
      font-size: 14px !important;
      line-height: 1.25 !important;
    }

    #content-root div[data-analyze] > svg,
    #content-root div[data-analyze] > canvas,
    #content-root div[data-analyze] > img,
    #content-root div[data-analyze] svg,
    #content-root div[data-analyze] canvas{
      display:block !important;
      margin: 0 auto !important;
    }

    #content-root div[data-analyze]{
      margin: 0 0 14px 0 !important;
      padding: 0 !important;
    }

    #content-root table{
      width: 100% !important;
      max-width: 100% !important;
      table-layout: fixed !important;
    }

    #content-root th,
    #content-root td{
      word-break: break-all !important;
      overflow-wrap: anywhere !important;
      white-space: normal !important;
      vertical-align: middle !important;
      line-height: 1.4 !important;
      padding-top: 8px !important;
      padding-bottom: 8px !important;
    }

    #content-root th{
      font-weight: 400 !important;
    }

    #content-root th strong,
    #content-root th b,
    #content-root td strong,
    #content-root td b{
      font-weight: 700 !important;
    }

    #content-root table p{
      margin: 0 !important;
      white-space: normal !important;
      word-break: break-all !important;
      overflow-wrap: anywhere !important;
    }

    #content-root .image-footnote-wrap{
      break-inside: avoid !important;
      page-break-inside: avoid !important;
    }

    #content-root{
      font-family: "NotoSansKR", system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif !important;
      -webkit-font-smoothing: antialiased !important;
      text-rendering: geometricPrecision !important;
      color:#000000 !important;
      -webkit-text-fill-color:#000000 !important;
      padding-bottom: 12px !important;
      overflow: visible !important;
    }

    #content-root *{
      box-sizing:border-box !important;
      caret-color:#000000 !important;
      opacity:1 !important;
      filter:none !important;
      text-shadow:none !important;
      mix-blend-mode:normal !important;
      -webkit-text-stroke:0 !important;
      overflow: visible !important;
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
      font-size: 40px !important;
      font-weight: 400 !important;
      line-height: 1.16 !important;
      margin: 0 0 12px 0 !important;
      padding-top: 3px !important;
    }

    #content-root h2{
      font-size: 32px !important;
      font-weight: 400 !important;
      line-height: 1.18 !important;
      margin: 0 0 12px 0 !important;
      padding-top: 3px !important;
    }

    #content-root h3{
      font-size: 26px !important;
      font-weight: 400 !important;
      line-height: 1.2 !important;
      margin: 0 0 10px 0 !important;
      padding-top: 3px !important;
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
      font-size: 18px !important;
      margin-top: 0 !important;
      margin-bottom: 10px !important;
      line-height: 1.5 !important;
      word-break: keep-all !important;
      overflow-wrap: break-word !important;
      padding-top: 2px !important;
    }

    #content-root p:empty{
      margin: 0 !important;
      padding: 0 !important;
      height: 0 !important;
    }

    #content-root li{
      font-size: 18px !important;
      line-height: 1.5 !important;
      word-break: keep-all !important;
      overflow-wrap: break-word !important;
      break-inside: auto !important;
      page-break-inside: auto !important;
      padding-top: 0 !important;
    }

    #content-root li[data-pdf-atomic-li="1"]{
      break-inside: avoid !important;
      page-break-inside: avoid !important;
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
      padding-left: 24px !important;
      margin: 0 0 6px 0 !important;
      display: block !important;
    }

    #content-root li[data-pdf-li="1"] > p{
      display: inline !important;
      margin: 0 !important;
      padding-top: 0 !important;
    }

    #content-root .__pdf_marker{
      position: absolute !important;
      left: 0 !important;
      top: 2px !important;
      width: 20px !important;
      height: auto !important;
      display: block !important;
      font-weight: 400 !important;
      font-size: 18px !important;
      line-height: 1.5 !important;
      color: #000000 !important;
      -webkit-text-fill-color:#000000 !important;
      white-space: nowrap !important;
      overflow: visible !important;
      padding-top: 0 !important;
      transform: translateY(0) !important;
    }

    #content-root .__pdf_li_content{
      min-width: 0 !important;
      display: block !important;
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
      display: flex !important;
      align-items: center !important;
      justify-content: flex-start !important;
      padding: 10px 14px 12px 14px !important;
      font-size: 14.5px !important;
      line-height: 1.35 !important;
      text-align: inherit !important;
      position: relative !important;
      z-index: 1 !important;
    }

    #content-root tbody td .__pdf_cell{
      white-space: normal !important;
      word-break: keep-all !important;
      overflow-wrap: anywhere !important;
      text-align: inherit !important;
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

  const ensureAnalyzeTitleCaption = (root: HTMLElement) => {
    const blocks = Array.from(
      root.querySelectorAll(
        'div[data-analyze], div[data-title], div[data-footnote]',
      ),
    ) as HTMLElement[];

    for (const el of blocks) {
      const debug =
        typeof window !== 'undefined' &&
        (window as unknown as { __REPORT_DEBUG__?: boolean })
          .__REPORT_DEBUG__ === true;
      const hasChart = !!el.querySelector('svg, canvas, img');
      if (!hasChart && !el.hasAttribute('data-analyze')) continue;

      const title = (el.getAttribute('data-title') ?? '').trim();
      const footnote = (el.getAttribute('data-footnote') ?? '').trim();
      const tfidfWrapper = el.closest(
        'div[data-analyze-wrapper="tfidf"]',
      ) as HTMLElement | null;
      const isNetwork =
        el.getAttribute('data-analyze') === 'network' ||
        el.querySelector('.network-wrap, canvas') != null;
      const isTfidf =
        !!tfidfWrapper || el.querySelector('.tfidf-svg, .tfidf-wrap') != null;

      if (debug && (isTfidf || isNetwork)) {
        console.log('[report-pdf] block:start', {
          kind: el.getAttribute('data-analyze'),
          hasChart,
          title,
          footnote,
          tfidfWrapper: !!tfidfWrapper,
          tfidfTitles: el.querySelectorAll(
            '.tfidf-title, [data-analyze-title="tfidf"]',
          ).length,
          tfidfFootnotes: el.querySelectorAll('.tfidf-footnote').length,
          networkFootnotes: el.querySelectorAll('.network-footnote').length,
        });
      }

      if (tfidfWrapper) {
        tfidfWrapper
          .querySelectorAll(
            ':scope > .__pdf_analyze_title, :scope > .tfidf-title, :scope > [data-analyze-title="tfidf"]',
          )
          .forEach((n) => n.remove());
      }

      el.querySelectorAll(
        ':scope > .__pdf_analyze_title, :scope > .__pdf_analyze_caption',
      ).forEach((n) => n.remove());
      el.querySelectorAll('.tfidf-title, [data-analyze-title="tfidf"]').forEach(
        (n) => n.remove(),
      );
      el.querySelectorAll('.tfidf-footnote').forEach((n) => n.remove());
      if (isNetwork) {
        el.querySelectorAll('.network-footnote').forEach((n) => n.remove());
      }

      if (isTfidf) {
        const tfidfRoot = tfidfWrapper ?? el;
        tfidfRoot
          .querySelectorAll('.tfidf-title, [data-analyze-title="tfidf"]')
          .forEach((n) => n.remove());
        const parent = tfidfWrapper?.parentElement ?? el.parentElement;
        const sibling = (tfidfWrapper ?? el).nextElementSibling;
        if (sibling && sibling.classList.contains('tfidf-footnote')) {
          sibling.remove();
        }
        parent
          ?.querySelectorAll(':scope > .tfidf-footnote')
          .forEach((n) => n.remove());
      }

      if (title.length > 0) {
        const t = root.ownerDocument.createElement('div');
        t.className = '__pdf_analyze_title';
        t.textContent = title;
        if (tfidfWrapper) {
          tfidfWrapper.insertBefore(t, tfidfWrapper.firstChild);
        } else {
          el.insertBefore(t, el.firstChild);
        }
      }

      if (footnote.length > 0) {
        const c = root.ownerDocument.createElement('div');
        c.className = '__pdf_analyze_caption';
        c.textContent = footnote;
        if (tfidfWrapper) {
          tfidfWrapper.appendChild(c);
        } else {
          el.appendChild(c);
        }
      }

      if (debug && (isTfidf || isNetwork)) {
        const holder = tfidfWrapper ?? el;
        console.log('[report-pdf] block:end', {
          kind: el.getAttribute('data-analyze'),
          pdfTitles: holder.querySelectorAll('.__pdf_analyze_title').length,
          pdfCaptions: holder.querySelectorAll('.__pdf_analyze_caption').length,
          tfidfTitles: holder.querySelectorAll(
            '.tfidf-title, [data-analyze-title="tfidf"]',
          ).length,
          tfidfFootnotes: holder.querySelectorAll('.tfidf-footnote').length,
          networkFootnotes: holder.querySelectorAll('.network-footnote').length,
        });
        if (isTfidf) {
          const box = holder.getBoundingClientRect();
          console.log(
            '[report-pdf] tfidf:box',
            box.height,
            box.top,
            box.bottom,
          );
          const titleEl = holder.querySelector(
            '.__pdf_analyze_title',
          ) as HTMLElement | null;
          const wrapEl = holder.querySelector(
            '.tfidf-wrap',
          ) as HTMLElement | null;
          const svgEl = holder.querySelector(
            'svg.tfidf-svg',
          ) as SVGElement | null;
          const captionEl = holder.querySelector(
            '.__pdf_analyze_caption',
          ) as HTMLElement | null;
          console.log('[report-pdf] tfidf:parts', {
            title: titleEl?.getBoundingClientRect?.().height ?? null,
            wrap: wrapEl?.getBoundingClientRect?.().height ?? null,
            svg: svgEl?.getBoundingClientRect?.().height ?? null,
            caption: captionEl?.getBoundingClientRect?.().height ?? null,
          });
          console.log('[report-pdf] tfidf:svg?', !!holder.querySelector('svg'));
          console.log('[report-pdf] tfidf:html', holder.innerHTML);
        }
      }

      el.setAttribute('data-pdf-keep', '1');
      el.setAttribute('data-pdf-atomic', '1');

      const s = el.style as CSSStyleDeclaration;
      s.breakInside = 'avoid';
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (s as any).pageBreakInside = 'avoid';
    }
  };

  const wrapImageFootnotes = (root: HTMLElement) => {
    const images = Array.from(root.querySelectorAll('img')) as HTMLElement[];
    for (const img of images) {
      const parent = img.parentElement;
      if (!parent) continue;
      const next = img.nextElementSibling as HTMLElement | null;
      if (!next || !next.classList.contains('image-footnote')) continue;
      if (parent.classList.contains('image-footnote-wrap')) continue;

      const wrap = root.ownerDocument.createElement('div');
      wrap.className = 'image-footnote-wrap';
      wrap.setAttribute('data-pdf-keep', '1');
      wrap.setAttribute('data-pdf-atomic', '1');

      parent.insertBefore(wrap, img);
      wrap.appendChild(img);
      wrap.appendChild(next);
    }
  };

  const materializeListMarkers = (root: HTMLElement) => {
    const lists = Array.from(root.querySelectorAll('ul, ol')) as (
      | HTMLUListElement
      | HTMLOListElement
    )[];
    for (const list of lists) {
      const isOl = list.tagName === 'OL';
      const startAttr = Number(list.getAttribute('start') ?? '1');
      const startIndex =
        Number.isFinite(startAttr) && startAttr > 0 ? startAttr : 1;
      const lis = Array.from(
        list.querySelectorAll(':scope > li'),
      ) as HTMLLIElement[];

      for (let index = 0; index < lis.length; index += 1) {
        const li = lis[index];
        li.setAttribute('data-pdf-li', '1');

        const already = li.querySelector(
          ':scope > .__pdf_marker',
        ) as HTMLSpanElement | null;
        if (already) continue;

        const marker = already ?? root.ownerDocument.createElement('span');
        marker.className = '__pdf_marker';
        if (!already) {
          if (isOl) {
            const valueRaw = li.getAttribute('value');
            const valueAttr =
              valueRaw && valueRaw.trim().length > 0
                ? Number.parseInt(valueRaw, 10)
                : Number.NaN;
            const value = Number.isFinite(valueAttr)
              ? valueAttr
              : startIndex + index;
            marker.textContent = `${value}.`;
          } else {
            marker.textContent = '•';
          }
          li.insertBefore(marker, li.firstChild);
        }

        const existingContent = li.querySelector(
          ':scope > .__pdf_li_content',
        ) as HTMLSpanElement | null;
        if (existingContent) continue;

        const contentWrap = root.ownerDocument.createElement('span');
        contentWrap.className = '__pdf_li_content';

        const nodes = Array.from(li.childNodes);
        for (const node of nodes) {
          if (node === marker) continue;
          contentWrap.appendChild(node);
        }
        li.appendChild(contentWrap);
      }
    }
  };

  const markAtomicBlocks = (root: HTMLElement) => {
    const atomicRoots = new Set<HTMLElement>();

    const isNestedInKeptBlock = (el: Element) => {
      const keep = el.closest('[data-pdf-keep="1"], [data-pdf-atomic="1"]');
      return !!keep && keep !== el;
    };

    const candidates = Array.from(
      root.querySelectorAll(
        [
          'div[data-analyze]',
          'div[data-title]',
          'div[data-footnote]',
          '[data-pdf-keep]',
          '.table-footnote-wrap',
          '.recharts-wrapper',
          '.recharts-responsive-container',
          '.chart',
          '.graph',
          '[class*="chart"]',
          '[class*="graph"]',
          'table',
          'canvas',
          'svg',
          'figure',
          'img',
          'pre',
          'blockquote',
          'h1,h2,h3,h4,h5,h6',
        ].join(','),
      ),
    ) as HTMLElement[];

    for (const el of candidates) {
      if (isNestedInKeptBlock(el)) continue;

      const container =
        (el.closest('div[data-analyze]') as HTMLElement | null) ??
        (el.closest('[data-pdf-keep="1"]') as HTMLElement | null) ??
        (el.closest('div[data-title]') as HTMLElement | null) ??
        (el.closest('div[data-footnote]') as HTMLElement | null) ??
        (el.closest('.recharts-responsive-container') as HTMLElement | null) ??
        (el.closest('.recharts-wrapper') as HTMLElement | null) ??
        (el.closest(
          '.chart, .graph, [class*="chart"], [class*="graph"]',
        ) as HTMLElement | null) ??
        (el.closest('figure') as HTMLElement | null) ??
        el;

      const parentAtomic = container.parentElement?.closest?.(
        '[data-pdf-atomic="1"]',
      ) as HTMLElement | null;
      if (parentAtomic && parentAtomic !== container) continue;

      atomicRoots.add(container);
    }

    for (const a of atomicRoots) {
      const parentAtomic = a.parentElement?.closest?.(
        '[data-pdf-atomic="1"]',
      ) as HTMLElement | null;
      if (parentAtomic && parentAtomic !== a) continue;

      a.setAttribute('data-pdf-atomic', '1');
      const s = a.style as CSSStyleDeclaration;
      s.breakInside = 'avoid';
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (s as any).pageBreakInside = 'avoid';
    }
  };

  const removeEmptyParagraphs = (root: HTMLElement) => {
    const ps = Array.from(root.querySelectorAll('p')) as HTMLParagraphElement[];
    for (const p of ps) {
      if (p.querySelector('img, table, svg, canvas, figure, a')) continue;

      const text = (p.textContent ?? '').replace(/\u00A0/g, ' ').trim();
      const onlyBr =
        p.childNodes.length === 1 &&
        p.firstChild?.nodeType === Node.ELEMENT_NODE &&
        (p.firstChild as Element).tagName === 'BR';

      if (text.length === 0 || onlyBr) p.remove();
    }
  };

  const insertPageBreakSpacers = (root: HTMLElement) => {
    const clearOld = Array.from(root.querySelectorAll('[data-pdf-spacer="1"]'));
    for (const n of clearOld) n.remove();

    const getBlocks = () => {
      const blocks = Array.from(
        root.querySelectorAll(
          ['[data-pdf-atomic="1"]', 'table', 'li[data-pdf-li="1"]'].join(','),
        ),
      ) as HTMLElement[];

      const out: HTMLElement[] = [];
      const uniq = new Set<HTMLElement>();

      for (const el of blocks) {
        const parentAtomic = el.parentElement?.closest?.(
          '[data-pdf-atomic="1"]',
        ) as HTMLElement | null;
        if (parentAtomic && parentAtomic !== el) continue;

        const r = el.getBoundingClientRect();
        if (r.width <= 0 || r.height <= 0) continue;

        if (!uniq.has(el)) {
          uniq.add(el);
          out.push(el);
        }
      }

      out.sort(
        (a, b) => a.getBoundingClientRect().top - b.getBoundingClientRect().top,
      );
      return out;
    };

    const maxIter = 12;
    const safetyPx = 12;

    for (let iter = 0; iter < maxIter; iter += 1) {
      let changed = false;

      const blocks = getBlocks();
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

        const isLi =
          el.tagName === 'LI' && el.getAttribute('data-pdf-li') === '1';
        const remaining = pageInnerHeightPx - mod;
        let lineHeightPx = 0;
        if (isLi) {
          const cs = getComputedStyle(el);
          const lh = parseFloat(cs.lineHeight);
          if (Number.isFinite(lh) && lh > 0) {
            lineHeightPx = lh;
          } else {
            const fs = parseFloat(cs.fontSize);
            lineHeightPx = Number.isFinite(fs) && fs > 0 ? fs * 1.5 : 24;
          }
        }

        const liNeedsSpacer =
          isLi && overflow > 0 && remaining < lineHeightPx * 1.2;
        const blockNeedsSpacer = !isLi && overflow > 6;

        if (liNeedsSpacer || blockNeedsSpacer) {
          const spacerH = pageInnerHeightPx - mod + safetyPx;
          if (spacerH < 8) continue;

          const prev = el.previousElementSibling as HTMLElement | null;
          if (prev?.getAttribute('data-pdf-spacer') === '1') continue;

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
            const align = clonedDoc.defaultView?.getComputedStyle(el).textAlign;
            if (align) {
              wrap.style.textAlign = align;
              if (align === 'center') {
                wrap.style.justifyContent = 'center';
              } else if (align === 'right') {
                wrap.style.justifyContent = 'flex-end';
              } else {
                wrap.style.justifyContent = 'flex-start';
              }
            }

            while (el.firstChild) {
              wrap.appendChild(el.firstChild);
            }
            el.appendChild(wrap);
          }

          removeEmptyParagraphs(root);
          ensureAnalyzeTitleCaption(root);
          wrapImageFootnotes(root);
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

export const buildReportPdfBlobFromContents = async (
  htmlContents: string,
): Promise<Blob> => {
  const { buildReportHtmlDocument } = await import('./report-html');
  const htmlDocument = buildReportHtmlDocument(htmlContents);
  return buildReportPdfBlob(htmlDocument);
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
