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

  return (
    <NodeViewWrapper className="relative my-4">
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
      <table className={className} data-footnote={footnote || undefined}>
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
