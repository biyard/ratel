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
  const t = ctrl.t;

  return (
    <>
      <Col>
        <Col className="gap-2 mb-4">
          <Row className="gap-5">
            <div className="flex flex-row w-fit gap-5 items-center">
              <div className="text-sm font-medium whitespace-nowrap flex-shrink-0">
                {t('total_quotas')}
              </div>
              <PanelQuotas
                quotas={ctrl.panel.quotas}
                canEdit={true}
                setQuotas={async (quota: number) => {
                  await ctrl.handleUpdateQuota(quota);
                }}
              />
            </div>
            <div className="flex flex-row w-full gap-5 items-center">
              <div className="text-sm font-medium whitespace-nowrap flex-shrink-0">
                {t('attribute_groups')}
              </div>
              <PanelLabels
                t={t}
                canEdit={true}
                values={ctrl.panel.attributes}
                setValues={async (v: PanelAttribute[]) => {
                  await ctrl.handleUpdateValues(v);
                }}
                placeholder={t('attribute_groups')}
              />
            </div>
          </Row>

          <PanelTable
            t={t}
            canEdit={true}
            panel_quotas={ctrl.panel.panel_quotas}
            onChangeQuota={ctrl.handleUpdateAttributeQuota}
            onDelete={ctrl.handleDeleteAttributeQuota}
          />
        </Col>
      </Col>
    </>
  );
}
