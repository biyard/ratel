import React, { useMemo } from 'react';
import { Node, mergeAttributes } from '@tiptap/core';
import { NodeViewWrapper, ReactNodeViewRenderer } from '@tiptap/react';
import { useTranslation } from 'react-i18next';
import { LdaTopicTable } from '@/features/spaces/components/topic/lda-topic-table';
import { X } from 'lucide-react';
import { Input } from '@/components/ui/input';

type Payload = {
  ldaTopics?: unknown;
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
function LdaNodeView(props: any) {
  const { t } = useTranslation('SpacePollAnalyze');
  const payload = useMemo(
    () => decode(props?.node?.attrs?.payload) as Payload | null,
    [props?.node?.attrs?.payload],
  );
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
          <Input
            type="text"
            value={footnote}
            onChange={(event) =>
              props?.updateAttributes?.({ footnote: event.target.value })
            }
            placeholder={t('lda_footnote_placeholder')}
            className="mb-2 w-full text-left rounded-md border border-input-box-border bg-transparent px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground"
          />
        ) : footnote ? (
          <div className="mb-2 w-full text-left text-xs text-muted-foreground">
            {footnote}
          </div>
        ) : null}
        <LdaTopicTable
          t={t}
          isHtml={true}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          ldaTopics={(payload as any)?.ldaTopics}
        />
      </div>
    </NodeViewWrapper>
  );
}

export const AnalyzeLdaBlock = Node.create({
  name: 'analyzeLda',
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

  parseHTML() {
    return [{ tag: 'div[data-analyze="lda"]' }];
  },

  renderHTML({ HTMLAttributes }) {
    return ['div', mergeAttributes(HTMLAttributes, { 'data-analyze': 'lda' })];
  },

  addNodeView() {
    return ReactNodeViewRenderer(LdaNodeView);
  },

  addCommands() {
    return {
      insertLdaBlock:
        (payload: Payload) =>
        ({ editor }) => {
          const encoded = encode(payload);
          return editor
            .chain()
            .focus()
            .insertContent([
              { type: this.name, attrs: { payload: encoded, footnote: '' } },
              { type: 'paragraph' },
            ])
            .run();
        },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any;
  },
});
