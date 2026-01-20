import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Edit1, Resizing, Save } from '@/components/icons';
import { PostEditorWithFooter } from '@/features/posts/components/post-editor-with-footer';
import { Editor } from '@tiptap/react';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';
import { Button } from '@/components/ui/button';
import { config } from '@/config';
import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { getUserMembership } from '@/lib/api/ratel/me.v3';
import { ReportPdfViewer } from './report-pdf-viewer';

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
  const [editorHeight, setEditorHeight] = React.useState(560);
  const [pdfViewerUrl, setPdfViewerUrl] = React.useState<string>('');
  const [isPdfViewerOpen, setIsPdfViewerOpen] = React.useState(false);
  const [pendingViewerOpen, setPendingViewerOpen] = React.useState(false);
  const resizeState = useRef<{ startY: number; startHeight: number } | null>(
    null,
  );

  const hasLda =
    Array.isArray(analyze?.lda_topics) && analyze.lda_topics.length > 0;
  const hasNetwork =
    analyze?.network != null &&
    Array.isArray(analyze?.network?.nodes) &&
    analyze.network.nodes.length > 0;
  const hasTfIdf = Array.isArray(analyze?.tf_idf) && analyze.tf_idf.length > 0;
  const showDownload = (hasLda || hasNetwork || hasTfIdf) && config.experiment;

  const { data: membership } = useQuery({
    queryKey: ['user-membership'],
    queryFn: getUserMembership,
  });
  const tierName = String(membership?.tier ?? '');
  const isPaidMember = tierName.length > 0 && !tierName.includes('Free');

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
    if (pendingViewerOpen && url.startsWith('http')) {
      setPdfViewerUrl(url);
      setIsPdfViewerOpen(true);
      setPendingViewerOpen(false);
    }
  }, [analyze?.metadata_url, downloadToken, isDownloading, pendingViewerOpen]);

  const startEdit = () => setEditing(true);

  const save = () => {
    handleUpdateHtmlContents?.(content);
    setEditing(false);
  };

  const insertLda = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertLdaBlock({
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      ldaTopics: (analyze as any)?.lda_topics,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      htmlContents: (analyze as any)?.lda_html,
    });
  };

  const insertNetwork = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertNetworkBlock({
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      network: (analyze as any)?.network,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      htmlContents: (analyze as any)?.network_html,
    });
  };

  const insertTfidf = () => {
    const ed = editorRef.current;
    if (!ed) return;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ed.commands as any).insertTfidfBlock({
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      tf_idf: (analyze as any)?.tf_idf,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      htmlContents: (analyze as any)?.tfidf_html,
    });
  };

  const onDownload = async () => {
    const existingUrl = String(analyze?.metadata_url ?? '');
    if (existingUrl.startsWith('http')) {
      setPdfViewerUrl(existingUrl);
      setIsPdfViewerOpen(true);
      return;
    }

    try {
      setDownloadToken(String(analyze?.metadata_url ?? ''));
      setIsDownloading(true);
      setPendingViewerOpen(true);
      await Promise.resolve(handleDownloadAnalyze?.());
    } catch {
      setIsDownloading(false);
      setPendingViewerOpen(false);
    }
  };

  useEffect(() => {
    const onMove = (event: MouseEvent) => {
      if (!resizeState.current) return;
      const delta = event.clientY - resizeState.current.startY;
      const next = resizeState.current.startHeight + delta;
      const min = 360;
      const max = Math.max(min, window.innerHeight * 2);
      setEditorHeight(Math.max(min, Math.min(max, next)));
    };

    const onUp = () => {
      if (resizeState.current) {
        resizeState.current = null;
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      }
    };

    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
    return () => {
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
  }, []);

  return (
    <div className="flex flex-col w-full">
      <ReportPdfViewer
        open={isPdfViewerOpen}
        url={pdfViewerUrl}
        fileName="analysis-report.pdf"
        spacePk={String(analyze?.pk ?? '')}
        analyzePk={String(analyze?.sk ?? '')}
        enableAi={isPaidMember}
        onClose={() => setIsPdfViewerOpen(false)}
      />
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
      <div
        className="w-full rounded-lg bg-card p-6 flex flex-col min-h-0 overflow-hidden"
        style={{ height: `${editorHeight}px` }}
      >
        <div className="flex items-center justify-end flex-shrink-0">
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
        <div className="flex flex-col w-full min-h-0 flex-1 overflow-hidden">
          <PostEditorWithFooter
            ref={editorRef}
            content={content}
            onUpdate={setContent}
            placeholder={t('report_draft_editor_placeholder')}
            editing={editing}
            enableTableFootnote={true}
            enableImageFootnote={true}
            toolbarFooter={
              editing && (hasLda || hasNetwork || hasTfIdf) ? (
                <div className="flex flex-row gap-2 w-full justify-end items-center">
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!hasLda}
                    onClick={insertLda}
                  >
                    {t('insert_lda')}
                  </Button>
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!hasNetwork}
                    onClick={insertNetwork}
                  >
                    {t('insert_text_network')}
                  </Button>
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!hasTfIdf}
                    onClick={insertTfidf}
                  >
                    {t('insert_tf_idf')}
                  </Button>
                </div>
              ) : null
            }
          />
        </div>

        <div className="flex items-center justify-end pt-2 flex-shrink-0">
          <button
            type="button"
            aria-label="Resize editor height"
            className="cursor-ns-resize select-none"
            onMouseDown={(event) => {
              event.preventDefault();
              resizeState.current = {
                startY: event.clientY,
                startHeight: editorHeight,
              };
              document.body.style.cursor = 'ns-resize';
              document.body.style.userSelect = 'none';
            }}
          >
            <Resizing />
          </button>
        </div>
      </div>
    </div>
  );
}
