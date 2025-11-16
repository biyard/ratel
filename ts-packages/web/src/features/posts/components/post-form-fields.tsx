import { Loader2, Check, Clock } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';
import { PostEditor } from './post-editor';
import { Editor } from '@tiptap/core';
import { EditorStatus } from './create-post-page/use-create-post-page-controller';
import { useCreatePostPageI18n } from './create-post-page/i18n';

export interface PostFormFieldsProps {
  // Title
  title: string;
  titleMaxLength: number;
  onTitleChange: (e: React.ChangeEvent<HTMLInputElement>) => void;

  // Content
  content: string | null;
  onContentUpdate: (html: string) => void;
  editorRef: React.RefObject<Editor | null>;

  // Image
  imageUrl: string | null;
  onImageUpload?: (imageUrl: string) => Promise<void>;
  onRemoveImage?: () => void;

  // Status
  status: EditorStatus;
  lastSavedAt: Date | null;
  isModified: boolean;
  formatLastSaved: (date: Date | null) => string;

  // i18n
  t: ReturnType<typeof useCreatePostPageI18n>;
  disabledImageUpload?: boolean;
}

export function PostFormFields(props: PostFormFieldsProps) {
  const {
    title,
    titleMaxLength,
    onTitleChange,
    content,
    onContentUpdate,
    editorRef,
    imageUrl,
    onImageUpload,
    onRemoveImage,
    status,
    lastSavedAt,
    isModified,
    formatLastSaved,
    disabledImageUpload = false,
    t,
  } = props;

  return (
    <>
      <Col>
        {/* Title Input */}
        <div className="relative">
          <Input
            id="post-title-input"
            type="text"
            placeholder={t.title_placeholder}
            value={title}
            onChange={onTitleChange}
            className="w-full text-text-primary bg-input-box-bg border-input-box-border dark:bg-[#101010] placeholder:text-gray-600"
          />
          {/* Red asterisk overlay on placeholder */}
          {!title && (
            <span className="absolute left-[2.7rem] top-1/2 -translate-y-1/2 text-[#EF4444] pointer-events-none">
              *
            </span>
          )}
          <div className="absolute right-3 top-1/2 text-sm -translate-y-1/2 text-neutral-400">
            {title.length}/{titleMaxLength}
          </div>
        </div>

        {/* Rich Text Editor - TipTap */}
        <div className="relative">
          <PostEditor
            disabledImageUpload={disabledImageUpload}
            ref={editorRef}
            editable={true}
            content={content || ''}
            onUpdate={onContentUpdate}
            placeholder={t.content_placeholder}
            data-pw="post-content-editor"
            minHeight="300px"
            url={imageUrl}
            onImageUpload={onImageUpload}
            onRemoveImage={onRemoveImage}
            className="dark:!bg-[#101010] [&_.tiptap-editor_.ProseMirror_p.is-editor-empty:first-child::before]:!text-gray-600"
            toolbarClassName="dark:!bg-[#101010] !border-b-0"
          />

          {/* Saving Status Indicator - positioned at bottom right of editor */}
          <div className="flex absolute right-3 bottom-3 gap-2 items-center py-1 px-2 text-xs rounded text-neutral-400 bg-card">
            {status === EditorStatus.Saving ? (
              <>
                <Loader2 className="animate-spin" size={14} />
                <span>{t.saving}</span>
              </>
            ) : isModified ? (
              <>
                <Clock size={14} className="text-yellow-500" />
                <span className="text-yellow-500">{t.unsaved_changes}</span>
              </>
            ) : lastSavedAt ? (
              <>
                <Check size={14} className="text-green-500" />
                <span className="text-green-500">{t.all_changes_saved}</span>
              </>
            ) : null}
          </div>
        </div>

        {lastSavedAt && (
          <Row className="justify-end items-center w-full text-xs text-neutral-400">
            {formatLastSaved(lastSavedAt)}
          </Row>
        )}
      </Col>
    </>
  );
}
