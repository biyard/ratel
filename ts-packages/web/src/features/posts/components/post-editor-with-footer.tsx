import { forwardRef } from 'react';
import { Editor } from '@tiptap/react';
import { PostEditor } from './post-editor';

type PostEditorWithFooterProps = {
  content: string;
  onUpdate: (content: string) => void;
  placeholder?: string;
  editing: boolean;
  toolbarFooter?: React.ReactNode;
  enableTableFootnote?: boolean;
  enableImageFootnote?: boolean;
};

export const PostEditorWithFooter = forwardRef<
  Editor | null,
  PostEditorWithFooterProps
>(
  (
    {
      content,
      onUpdate,
      placeholder,
      editing,
      toolbarFooter,
      enableTableFootnote,
      enableImageFootnote,
    },
    ref,
  ) => (
    <PostEditor
      ref={ref}
      url={null}
      content={content}
      onUpdate={onUpdate}
      placeholder={placeholder}
      minHeight="0px"
      showToolbar={editing}
      showBubbleToolbar={false}
      editable={editing}
      disabledFileUpload
      disabledImageUpload={false}
      containerClassName="h-full min-h-0 overflow-hidden"
      className="h-full min-h-0 overflow-hidden"
      editorClassName="flex-1 min-h-0 overflow-y-auto"
      toolbarFooter={toolbarFooter}
      enableTableFootnote={enableTableFootnote}
      enableImageFootnote={enableImageFootnote}
      enabledFeatures={{
        bold: true,
        italic: true,
        underline: true,
        strike: false,
        textColor: true,
        highlight: false,
        heading: true,
        align: true,
        lists: true,
        link: false,
        video: false,
      }}
    />
  ),
);

PostEditorWithFooter.displayName = 'PostEditorWithFooter';
