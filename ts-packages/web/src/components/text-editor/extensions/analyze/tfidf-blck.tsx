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
        <TfIdfChart
          t={t}
          isHtml={true}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          tf_idf={(payload as any)?.tf_idf}
        />
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
        parseHTML: (el) => (el as HTMLElement).getAttribute('data-payload'),
        renderHTML: (attrs) => {
          if (!attrs.payload) return {};
          return { 'data-payload': attrs.payload };
        },
      },
    };
  },

  parseHTML() {
    return [{ tag: 'div[data-analyze="tfidf"]' }];
  },

  renderHTML({ HTMLAttributes }) {
    return [
      'div',
      mergeAttributes(HTMLAttributes, { 'data-analyze': 'tfidf' }),
    ];
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
              { type: this.name, attrs: { payload: encoded } },
              { type: 'paragraph' },
            ])
            .run();
        },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any;
  },
});
