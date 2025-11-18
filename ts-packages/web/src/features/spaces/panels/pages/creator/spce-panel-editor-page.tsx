import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelEditorController } from './use-space-panel-editor-controller';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { PanelQuotas } from '../../components/panel_quota';
import { PanelTable } from '../../components/panel_table';
import { MultiSelect } from '@/components/ui/multi-select';

export function SpacePanelEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpacePanelEditorController(spacePk);
  const t = ctrl.t;

  return (
    <>
      <Col>
        <Col className="gap-2 mb-4">
          <Row className="gap-5">
            <div className="flex flex-row gap-5 items-center w-fit">
              <div className="shrink-0 text-sm font-medium whitespace-nowrap">
                {t('total_quotas')}
              </div>
              <PanelQuotas
                quotas={ctrl.space.quota}
                canEdit={true}
                setQuotas={async (quota: number) => {
                  await ctrl.handleUpdateQuota(quota);
                }}
              />
            </div>
            <div className="flex flex-row gap-5 items-center grow">
              <div className="shrink-0 text-sm font-medium whitespace-nowrap">
                {t('attribute_groups')}
              </div>
              <MultiSelect
                className="grow"
                options={ctrl.allOptions}
                value={ctrl.selectedAttribute}
                onChange={ctrl.handleChangeSelectedAttributes}
                placeholder={t('attribute_groups')}
              />
            </div>
          </Row>

          <PanelTable
            t={t}
            canEdit={true}
            panels={ctrl.panels}
            onChangeQuota={ctrl.handleUpdateAttributeQuota}
            onDelete={ctrl.handleDeleteAttributeQuota}
          />
        </Col>
      </Col>
    </>
  );
}
