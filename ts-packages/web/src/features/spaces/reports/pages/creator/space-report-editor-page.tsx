import { ReportDraft } from '@/features/spaces/reports/components/report-draft';
import { Col } from '@/components/ui/col';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpaceReportEditorController } from './use-space-report-editor-controller';

export function SpaceReportEditorPage({ spacePk }: SpacePathProps) {
  const { data: space } = useSpaceById(spacePk);
  const ctrl = useSpaceReportEditorController(spacePk);

  if (ctrl.isLoading || !ctrl.analyze) {
    return (
      <Col className="gap-4">
        <div className="flex items-center justify-center p-8">
          <p className="text-muted-foreground">Loading...</p>
        </div>
      </Col>
    );
  }

  const canEdit = space?.isAdmin() ?? false;
  const canView = space?.participated || space?.isAdmin();

  return (
    <Col className="gap-4">
      <ReportDraft
        analyze={ctrl.analyze}
        handleUpdateHtmlContents={ctrl.handleUpdateHtmlContents}
        onDownload={ctrl.handleDownload}
        canEdit={canEdit}
        canView={canView}
      />
    </Col>
  );
}
