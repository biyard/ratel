import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelEditorController } from './use-space-panel-editor-controller';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { PanelQuotas } from '../../components/panel_quota';
import { PanelLabels } from '../../components/panel_labels';
import { PanelAttribute } from '../../types/panel-attribute';
import { PanelTable } from '../../components/panel_table';

export function SpacePanelEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpacePanelEditorController(spacePk);

  return (
    <>
      <Col>
        <Col className="gap-2 mb-4">
          <Row className="gap-5">
            <div className="flex flex-row w-fit gap-5 items-center">
              <div className="text-sm font-medium">Total Quotas</div>
              <PanelQuotas
                quotas={ctrl.panel.quotas}
                canEdit={true}
                setQuotas={async (quota: number) => {
                  await ctrl.handleUpdateQuota(quota);
                }}
              />
            </div>
            <div className="flex flex-row w-full gap-5 items-center">
              <div className="text-sm font-medium">Attribute Groups</div>
              <PanelLabels
                canEdit={true}
                values={ctrl.panel.attributes}
                setValues={async (v: PanelAttribute[]) => {
                  await ctrl.handleUpdateValues(v);
                }}
                placeholder="Attribute Groups"
              />
            </div>
          </Row>

          <PanelTable
            canEdit={true}
            panel_quotas={ctrl.panel.panel_quotas}
            onChangeQuota={ctrl.handleUpdateAttributeQuota}
            onDelete={ctrl.handleDeleteAttributeQuota}
          />
          {/* <PanelTable
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
          /> */}
        </Col>
      </Col>
    </>
  );
}
