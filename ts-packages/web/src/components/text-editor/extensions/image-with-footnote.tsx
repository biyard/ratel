import React from 'react';
import Image from '@tiptap/extension-image';
import { NodeViewWrapper, ReactNodeViewRenderer } from '@tiptap/react';
import { useTranslation } from 'react-i18next';
import { Input } from '@/components/ui/input';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function ImageFootnoteView(props: any) {
  const { t } = useTranslation('SpacePollAnalyze');
  const footnote = String(props?.node?.attrs?.footnote ?? '');
  const src = String(props?.node?.attrs?.src ?? '');
  const alt = String(props?.node?.attrs?.alt ?? '');
  const className = String(props?.node?.attrs?.class ?? '');
  const isEditable = !!props?.editor?.isEditable;

  return (
    <NodeViewWrapper className="my-4 flex w-full flex-col items-center">
      <img src={src} alt={alt} className={className} />
      {isEditable ? (
        <Input
          type="text"
          value={footnote}
          onChange={(event) =>
            props?.updateAttributes?.({ footnote: event.target.value })
          }
          placeholder={t('image_footnote_placeholder')}
          className="mt-2 w-full rounded-md border border-input-box-border bg-transparent px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground text-center"
        />
      ) : footnote ? (
        <div className="mt-2 w-full text-center text-xs text-muted-foreground">
          {footnote}
        </div>
      ) : null}
    </NodeViewWrapper>
  );
}

export const ImageWithFootnote = Image.extend({
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
    return ReactNodeViewRenderer(ImageFootnoteView);
  },
});
