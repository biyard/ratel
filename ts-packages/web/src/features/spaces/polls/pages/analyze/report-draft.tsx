import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Edit1, Resizing, Save } from '@/components/icons';
import { PostEditor } from '@/features/posts/components/post-editor';
import { Editor } from '@tiptap/react';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';
import { Button } from '@/components/ui/button';
import { config } from '@/config';
import React from 'react';

type ReportDraftProps = {
  analyze?: SpaceAnalyze;
  handleUpdateHtmlContents?: (htmlContents: string) => void;
  handleDownloadAnalyze?: () => void | Promise<{ spacePk: string }>;
};

export function ReportDraft({
  analyze,
  handleUpdateHtmlContents,
  handleDownloadAnalyze,
}: ReportDraftProps) {
  const { t } = useTranslation('SpacePollAnalyze');

  const [content, setContent] = useState<string>(() => {
    const initial = String(analyze?.html_contents ?? '');
    return initial;
  });

  const [editing, setEditing] = useState(false);
  const editorRef = useRef<Editor | null>(null);
  const [isDownloading, setIsDownloading] = React.useState(false);
  const [downloadToken, setDownloadToken] = React.useState<string>('');

  const hasLda =
    Array.isArray(analyze?.lda_topics) && analyze.lda_topics.length > 0;
  const hasNetwork =
    analyze?.network != null &&
    Array.isArray(analyze?.network?.nodes) &&
    analyze.network.nodes.length > 0;
  const hasTfIdf = Array.isArray(analyze?.tf_idf) && analyze.tf_idf.length > 0;
  const showDownload = (hasLda || hasNetwork || hasTfIdf) && !config.experiment;

  useEffect(() => {
    if (editing) return;
    setContent(String(analyze?.html_contents ?? ''));
  }, [analyze?.html_contents, editing]);

  useEffect(() => {
    if (!isDownloading) return;
    const url = String(analyze?.metadata_url ?? '');
    if (url.startsWith('http') && url !== downloadToken) {
      setIsDownloading(false);
    }
  }, [analyze?.metadata_url, downloadToken, isDownloading]);

  const startEdit = () => setEditing(true);

  const save = () => {
    handleUpdateHtmlContents?.(content);
    setEditing(false);
  };

  const onDownload = async () => {
    const existingUrl = String(analyze?.metadata_url ?? '');
    if (existingUrl.startsWith('http')) {
      const a = document.createElement('a');
      a.href = existingUrl;
      a.target = '_blank';
      a.rel = 'noreferrer';
      a.download = '';
      document.body.appendChild(a);
      a.click();
      a.remove();
      return;
    }

    try {
      setDownloadToken(String(analyze?.metadata_url ?? ''));
      setIsDownloading(true);
      await Promise.resolve(handleDownloadAnalyze?.());
    } catch {
      setIsDownloading(false);
    }
  };

  return (
    <div className="flex flex-col w-full">
      {showDownload && (
        <div className="flex flex-row w-full justify-end mb-2.5">
          <Button
            variant="primary"
            onClick={onDownload}
            disabled={isDownloading}
          >
            {isDownloading ? t('downloading') : t('download_analyze')}
          </Button>
        </div>
      )}
      <div className="w-full rounded-lg bg-card p-6">
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
          <Resizing />
        </div>
      </div>
    </div>
  );
}
