import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Edit1, Save } from '@/components/icons';
import { PostEditor } from '@/features/posts/components/post-editor';
import { Editor } from '@tiptap/react';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';

type ReportDraftProps = {
  analyze?: SpaceAnalyze;
  handleUpdateHtmlContents?: (htmlContents: string) => void;
};

export function ReportDraft({
  analyze,
  handleUpdateHtmlContents,
}: ReportDraftProps) {
  const { t } = useTranslation('SpacePollAnalyze');

  const [content, setContent] = useState<string>(() => {
    const initial = String(analyze?.html_contents ?? '');
    return initial;
  });

  const [editing, setEditing] = useState(false);
  const editorRef = useRef<Editor | null>(null);

  useEffect(() => {
    if (editing) return;
    setContent(String(analyze?.html_contents ?? ''));
  }, [analyze?.html_contents, editing]);

  const startEdit = () => setEditing(true);

  const save = () => {
    handleUpdateHtmlContents?.(content);
    setEditing(false);
  };

  return (
    <div className="w-full rounded-lg bg-card p-6">
      <PostEditor
        ref={editorRef}
        url={null}
        content={content}
        onUpdate={setContent}
        placeholder={t('report_draft_editor_placeholder')}
        minHeight="320px"
        showToolbar={editing}
        editable={editing}
        disabledFileUpload
        disabledImageUpload
        enabledFeatures={{
          lda: true,
          network: true,
          tfidf: true,
        }}
        onClickLda={() => {
          const ed = editorRef.current;
          if (!ed) return;
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (ed.commands as any).insertLdaBlock({
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            ldaTopics: (analyze as any)?.lda_topics,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            htmlContents: (analyze as any)?.lda_html,
          });
        }}
        onClickNetwork={() => {
          const ed = editorRef.current;
          if (!ed) return;
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (ed.commands as any).insertNetworkBlock({
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            network: (analyze as any)?.network,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            htmlContents: (analyze as any)?.network_html,
          });
        }}
        onClickTfidf={() => {
          const ed = editorRef.current;
          if (!ed) return;
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (ed.commands as any).insertTfidfBlock({
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            tf_idf: (analyze as any)?.tf_idf,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            htmlContents: (analyze as any)?.tfidf_html,
          });
        }}
      />

      <div className="flex items-center justify-end">
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
    </div>
  );
}
