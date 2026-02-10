import { Edit1, Resizing, Save } from '@/components/icons';
import { PostEditorWithFooter } from '@/features/posts/components/post-editor-with-footer';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';
import { Button } from '@/components/ui/button';
import { useReportDraftController } from './use-report-draft-controller';

type ReportDraftProps = {
  analyze?: SpaceAnalyze;
  handleUpdateHtmlContents?: (htmlContents: string) => void;
  onDownload?: () => void;
  canEdit?: boolean;
  canView?: boolean;
};

export function ReportDraft({
  analyze,
  handleUpdateHtmlContents,
  onDownload,
  canEdit = false,
  canView = true,
}: ReportDraftProps) {
  const ctrl = useReportDraftController({
    analyze,
    handleUpdateHtmlContents,
    onDownload,
  });

  return (
    <div className="flex flex-col w-full">
      {ctrl.showDownload && canView && (
        <div className="flex flex-row w-full justify-end mb-2.5">
          <Button
            variant="primary"
            onClick={ctrl.handleDownload}
            disabled={ctrl.isDownloading}
          >
            {ctrl.isDownloading
              ? ctrl.t('downloading')
              : ctrl.t('download_analyze')}
          </Button>
        </div>
      )}
      <div
        className="w-full rounded-lg bg-card p-6 flex flex-col min-h-0 overflow-hidden"
        style={{ height: `${ctrl.editorHeight}px` }}
      >
        <div className="flex items-center justify-end flex-shrink-0">
          <div className="flex items-center gap-3">
            {canEdit && (
              <>
                {!ctrl.editing ? (
                  <Edit1
                    className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
                    onClick={ctrl.startEdit}
                  />
                ) : (
                  <Save
                    className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
                    onClick={ctrl.save}
                  />
                )}
              </>
            )}
          </div>
        </div>
        <div className="flex flex-col w-full min-h-0 flex-1 overflow-hidden">
          <PostEditorWithFooter
            ref={ctrl.editorRef}
            content={ctrl.content}
            onUpdate={ctrl.setContent}
            placeholder={ctrl.t('report_draft_editor_placeholder')}
            editing={ctrl.editing}
            enableTableFootnote={true}
            enableImageFootnote={true}
            toolbarFooter={
              ctrl.editing &&
              (ctrl.hasLda || ctrl.hasNetwork || ctrl.hasTfIdf) ? (
                <div className="flex flex-row gap-2 w-full justify-end items-center">
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!ctrl.hasLda}
                    onClick={ctrl.insertLda}
                  >
                    {ctrl.t('insert_lda')}
                  </Button>
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!ctrl.hasNetwork}
                    onClick={ctrl.insertNetwork}
                  >
                    {ctrl.t('insert_text_network')}
                  </Button>
                  <Button
                    type="button"
                    size="sm"
                    variant="rounded_secondary"
                    disabled={!ctrl.hasTfIdf}
                    onClick={ctrl.insertTfidf}
                  >
                    {ctrl.t('insert_tf_idf')}
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
            onMouseDown={ctrl.handleResizeMouseDown}
          >
            <Resizing />
          </button>
        </div>
      </div>
    </div>
  );
}
