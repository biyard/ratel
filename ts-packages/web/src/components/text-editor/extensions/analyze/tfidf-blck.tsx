import React, { useMemo } from 'react';
import { Node, mergeAttributes } from '@tiptap/core';
import { NodeViewWrapper, ReactNodeViewRenderer } from '@tiptap/react';
import { useTranslation } from 'react-i18next';
import { TfIdfChart } from '@/features/spaces/components/topic/tf-idf-chart';
import { X } from 'lucide-react';

type Payload = {
  tf_idf?: unknown;
  htmlContents?: string;
};

const encode = (v: unknown) => {
  const s = JSON.stringify(v ?? null);
  return typeof window === 'undefined'
    ? Buffer.from(s, 'utf8').toString('base64')
    : btoa(unescape(encodeURIComponent(s)));
};

const decode = (s?: string | null) => {
  if (!s) return null;
  try {
    const raw =
      typeof window === 'undefined'
        ? Buffer.from(s, 'base64').toString('utf8')
        : decodeURIComponent(escape(atob(s)));
    return JSON.parse(raw);
  } catch {
    return null;
  }
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function TfIdfNodeView(props: any) {
  const { t } = useTranslation('SpacePollAnalyze');
  const payload = useMemo(
    () => decode(props?.node?.attrs?.payload) as Payload | null,
    [props?.node?.attrs?.payload],
  );
  const title = String(props?.node?.attrs?.title ?? '');
  const footnote = String(props?.node?.attrs?.footnote ?? '');
  const isEditable = !!props?.editor?.isEditable;

  return (
    <NodeViewWrapper className="relative my-3">
      {isEditable && (
        <button
          type="button"
          className="absolute right-1 top-1 z-20 rounded-md p-1 hover:bg-muted"
          onClick={() => props?.deleteNode?.()}
        >
          <X className="h-4 w-4" />
        </button>
      )}
      <div
        className={
          isEditable
            ? 'rounded-lg border border-input-box-border bg-card p-4'
            : ''
        }
      >
        {isEditable ? (
          <input
            type="text"
            value={title}
            onChange={(event) =>
              props?.updateAttributes?.({ title: event.target.value })
            }
            placeholder={t('input_tf_idf_title_hint')}
            className="mb-3 w-full rounded-md border border-input-box-border bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground text-center"
          />
        ) : title ? (
          <div className="mb-3 w-full text-sm font-semibold text-center">
            {title}
          </div>
        ) : null}
        <TfIdfChart
          t={t}
          isHtml={true}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          tf_idf={(payload as any)?.tf_idf}
        />
        {isEditable ? (
          <input
            type="text"
            value={footnote}
            onChange={(event) =>
              props?.updateAttributes?.({ footnote: event.target.value })
            }
            placeholder={t('tfidf_footnote_placeholder')}
            className="mt-3 w-full rounded-md border border-input-box-border bg-transparent px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground text-center"
          />
        ) : footnote ? (
          <div className="mt-3 w-full text-xs text-muted-foreground text-center">
            {footnote}
          </div>
        ) : null}
      </div>
    </NodeViewWrapper>
  );
}

export const AnalyzeTfidfBlock = Node.create({
  name: 'analyzeTfidf',
  group: 'block',
  atom: true,
  isolating: true,

  addAttributes() {
    return {
      payload: {
        default: null,
        parseHTML: (el) => {
          const host = el as HTMLElement;
          const direct = host.getAttribute('data-payload');
          if (direct) return direct;
          const child = host.querySelector('div[data-analyze="tfidf"]');
          return child?.getAttribute('data-payload');
        },
        renderHTML: (attrs) => {
          if (!attrs.payload) return {};
          return { 'data-payload': attrs.payload };
        },
      },
      title: {
        default: '',
        parseHTML: (el) => {
          const host = el as HTMLElement;
          const direct = host.getAttribute('data-title');
          if (direct) return direct;
          const child = host.querySelector('[data-analyze-title="tfidf"]');
          return child?.textContent ?? '';
        },
        renderHTML: (attrs) => {
          if (!attrs.title) return {};
          return { 'data-title': attrs.title };
        },
      },
      footnote: {
        default: '',
        parseHTML: (el) => {
          const host = el as HTMLElement;
          const direct = host.getAttribute('data-footnote');
          if (direct) return direct;
          const child = host.querySelector('div[data-analyze="tfidf"]');
          return child?.getAttribute('data-footnote') ?? '';
        },
        renderHTML: (attrs) => {
          if (!attrs.footnote) return {};
          return { 'data-footnote': attrs.footnote };
        },
      },
    };
  },

  parseHTML() {
    return [
      { tag: 'div[data-analyze-wrapper="tfidf"]' },
      { tag: 'div[data-analyze="tfidf"]' },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    const { payload, title, ...rest } = HTMLAttributes;
    const chartAttrs: Record<string, string> = {
      'data-analyze': 'tfidf',
    };
    if (title) {
      chartAttrs['data-title'] = title as string;
    }
    const footnoteAttr =
      (HTMLAttributes as Record<string, unknown>)['data-footnote'] ??
      (HTMLAttributes as Record<string, unknown>).footnote;
    if (footnoteAttr) {
      chartAttrs['data-footnote'] = String(footnoteAttr);
    }
    if (payload) {
      chartAttrs['data-payload'] = payload as string;
    }

    if (title) {
      return [
        'div',
        mergeAttributes(rest, { 'data-analyze-wrapper': 'tfidf' }),
        ['div', { class: 'tfidf-title', 'data-analyze-title': 'tfidf' }, title],
        ['div', chartAttrs],
      ];
    }
    return ['div', mergeAttributes(rest, chartAttrs)];
  },

  addNodeView() {
    return ReactNodeViewRenderer(TfIdfNodeView);
  },

  addCommands() {
    return {
      insertTfidfBlock:
        (payload: Payload) =>
        ({ editor }) => {
          const encoded = encode(payload);
          return editor
            .chain()
            .focus()
            .insertContent([
              {
                type: this.name,
                attrs: { payload: encoded, title: '', footnote: '' },
              },
              { type: 'paragraph' },
            ])
            .run();
        },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any;
  },
});
