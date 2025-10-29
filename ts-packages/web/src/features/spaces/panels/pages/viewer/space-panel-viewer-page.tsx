import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelViewerController } from './use-space-panel-viewer-controller';
import { Col } from '@/components/ui/col';
import { PanelTable } from '../../components/panel_table';

export function SpacePanelViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelViewerPage: spacePk=${spacePk}`);
  const ctrl = useSpacePanelViewerController(spacePk);
  const panels = ctrl.panels.get();

  return (
    <>
      <Col>
        <Col className="gap-2 mb-4">
          <PanelTable
            panels={panels}
            t={ctrl.t}
            canEdit={false}
            onadd={() => {}}
            handleDeletePanel={() => {}}
            handleUpdateName={() => {}}
            handleUpdateQuotas={() => {}}
            openGenderPopup={() => {}}
            openAgePopup={() => {}}
            bookmark={ctrl.bookmark.get()}
            onloadmore={ctrl.loadMore}
          />
        </Col>
      </Col>
    </>
  );
}
