import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelEditorController } from './use-space-panel-editor-controller';
import { Col } from '@/components/ui/col';
import { PanelTable } from '../../components/panel_table';

export function SpacePanelEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpacePanelEditorController(spacePk);
  const panels = ctrl.panels.get();

  return (
    <>
      <Col>
        <Col className="gap-2 mb-4">
          <PanelTable
            panels={panels}
            t={ctrl.t}
            canEdit={true}
            onadd={ctrl.handleAddPanel}
            handleDeletePanel={ctrl.handleDeletePanel}
            handleUpdateName={ctrl.handleUpdateName}
            handleUpdateQuotas={ctrl.handleUpdateQuotas}
            openGenderPopup={ctrl.openGenderPopup}
            openAgePopup={ctrl.openAgePopup}
            bookmark={ctrl.bookmark.get()}
            onloadmore={ctrl.loadMore}
          />
        </Col>
      </Col>
    </>
  );
}
