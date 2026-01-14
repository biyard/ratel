import { useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Edit1, Save } from '@/components/icons';
import { PostEditor } from '@/features/posts/components/post-editor';
import { Editor } from '@tiptap/react';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';

type ReportDraftPanelProps = {
  analyze?: SpaceAnalyze;
};

export function ReportDraft({ analyze }: ReportDraftPanelProps) {
  const { t } = useTranslation('SpacePollAnalyze');
  const [content, setContent] = useState('');
  const [editing, setEditing] = useState(false);
  const editorRef = useRef<Editor | null>(null);

  console.log('analyze: ', analyze);

  const startEdit = () => {
    setEditing(true);
  };

  const save = () => {
    setEditing(false);
  };

  return (
    <div className="w-full rounded-lg bg-card p-6">
      <div className="flex items-center justify-between">
        <div className="text-base font-semibold text-foreground">
          {t('report_write')}
        </div>
        <div className="flex items-center gap-3">
          {!editing ? (
            <Edit1
              className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
              onClick={startEdit}
            />
          ) : (
            <Save
              className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
              onClick={save}
            />
          )}
        </div>
      </div>
      <PostEditor
        ref={editorRef}
        url={null}
        content={content}
        onUpdate={setContent}
        placeholder={t('report_draft_editor_placeholder')}
        minHeight="320px"
        showToolbar={editing}
        editable={editing}
        enabledFeatures={{ lda: true, network: true, tfidf: true }}
        disabledFileUpload
        disabledImageUpload
        onClickLda={() => {}}
        onClickNetwork={() => {}}
        onClickTfidf={() => {}}
      />
    </div>
  );
}
