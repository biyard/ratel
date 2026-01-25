import React from 'react';
import {
  NodeViewWrapper,
  NodeViewContent,
  ReactNodeViewRenderer,
} from '@tiptap/react';
import Table from '@tiptap/extension-table';
import { useTranslation } from 'react-i18next';
import { Input } from '@/components/ui/input';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function TableFootnoteView(props: any) {
  const { t } = useTranslation('SpacePollAnalyze');
  const footnote = String(props?.node?.attrs?.footnote ?? '');
  const isEditable = !!props?.editor?.isEditable;
  const htmlAttrs = props?.extension?.options?.HTMLAttributes ?? {};
  const className = [htmlAttrs.class, 'table-footnote-table']
    .filter(Boolean)
    .join(' ');
  const wrapRef = React.useRef<HTMLDivElement | null>(null);
  const tableRef = React.useRef<HTMLTableElement | null>(null);
  const cols = React.useMemo(() => {
    const node = props?.node;
    const firstRow = node?.firstChild;
    const widths: Array<number | null> = [];
    if (!firstRow) return widths;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    firstRow.forEach((cell: any) => {
      const colspan = Number(cell?.attrs?.colspan ?? 1) || 1;
      const colwidth = Array.isArray(cell?.attrs?.colwidth)
        ? cell.attrs.colwidth
        : [];
      for (let i = 0; i < colspan; i += 1) {
        const w = colwidth[i];
        widths.push(Number.isFinite(w) ? Number(w) : null);
      }
    });

    return widths;
  }, [props?.node]);

  React.useEffect(() => {
    const logSizes = () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const debug = (window as any)?.__TABLE_DEBUG__ === true;
      if (!debug) return;

      const wrapEl = wrapRef.current;
      const tableEl = tableRef.current;
      const tbodyEl = tableEl?.querySelector('tbody') ?? null;
      const innerEl =
        tbodyEl?.querySelector('[data-node-view-content-react]') ?? null;

      const info = (el: HTMLElement | null) => {
        if (!el) return null;
        const r = el.getBoundingClientRect();
        const cs = getComputedStyle(el);
        return {
          tag: el.tagName,
          className: el.className,
          width: r.width,
          display: cs.display,
          tableLayout: cs.tableLayout,
          minWidth: cs.minWidth,
          maxWidth: cs.maxWidth,
          overflowX: cs.overflowX,
        };
      };

      console.debug('[table-debug] sizes', {
        wrap: info(wrapEl),
        table: info(tableEl),
        tbody: info(tbodyEl as HTMLElement | null),
        inner: info(innerEl as HTMLElement | null),
      });
    };

    logSizes();
    const syncLayout = () => {
      const tableEl = tableRef.current;
      const tbodyEl = tableEl?.querySelector('tbody') ?? null;
      const innerEl =
        tbodyEl?.querySelector('[data-node-view-content-react]') ??
        (tbodyEl?.querySelector('div') as HTMLElement | null);
      if (tbodyEl) {
        tbodyEl.style.width = '100%';
        tbodyEl.style.minWidth = '100%';
      }
      if (innerEl instanceof HTMLElement) {
        innerEl.style.display = 'contents';
        innerEl.style.width = '100%';
      }
    };
    syncLayout();

    const handler = () => logSizes();
    window.addEventListener('table-debug', handler);
    return () => window.removeEventListener('table-debug', handler);
  }, []);

  return (
    <NodeViewWrapper
      ref={wrapRef}
      className="relative my-4 w-full table-footnote-wrap"
      data-pdf-keep="1"
    >
      {isEditable ? (
        <Input
          type="text"
          value={footnote}
          onChange={(event) =>
            props?.updateAttributes?.({ footnote: event.target.value })
          }
          placeholder={t('table_footnote_placeholder')}
          className="mb-2 w-full rounded-md border border-input-box-border bg-transparent px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground text-left"
        />
      ) : footnote ? (
        <div className="mb-2 w-full text-left text-xs text-muted-foreground">
          {footnote}
        </div>
      ) : null}
      <table
        ref={tableRef}
        className={className}
        data-footnote={footnote || undefined}
        style={{ width: '100%', minWidth: '100%', tableLayout: 'fixed' }}
      >
        {cols.length > 0 && (
          <colgroup>
            {cols.map((w, idx) => (
              <col
                key={`col-${idx}`}
                style={w ? { width: `${w}px` } : undefined}
              />
            ))}
          </colgroup>
        )}
        <NodeViewContent as="tbody" />
      </table>
    </NodeViewWrapper>
  );
}

export const TableWithFootnote = Table.extend({
  addAttributes() {
    return {
      ...this.parent?.(),
      footnote: {
        default: '',
        parseHTML: (el) =>
          (el as HTMLElement).getAttribute('data-footnote') ?? '',
        renderHTML: (attrs) => {
          if (!attrs.footnote) return {};
          return { 'data-footnote': attrs.footnote };
        },
      },
    };
  },
  addNodeView() {
    return ReactNodeViewRenderer(TableFootnoteView);
  },
});
