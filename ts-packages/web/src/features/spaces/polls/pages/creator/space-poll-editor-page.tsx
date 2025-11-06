import { logger } from '@/lib/logger';
import { useSpacePollEditorController } from './use-space-poll-editor-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import SurveyEditor from '@/features/spaces/components/survey/editor';
import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import { TimeRangeSetting } from '../../components/time-range-setting';
import Card from '@/components/card';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { SpaceType } from '@/features/spaces/types/space-type';

export function SpacePollEditorPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollEditorPage: spacePk=${spacePk}, pollPk=${pollPk}`);
  const ctrl = useSpacePollEditorController(spacePk, pollPk);
  const { t } = useTranslation('SpacePollEditor');

  return (
    <>
      <Col>
        <TimeRangeSetting
          canEdit={ctrl.space.isAdmin()}
          onChange={ctrl.onChangeTimeRange}
          startTimestampMillis={ctrl.poll.started_at}
          endTimestampMillis={ctrl.poll.ended_at}
          className="justify-end"
        />

        {ctrl.space.isAdmin() && ctrl.space.isDraft && (
          <div className="flex flex-row items-center gap-3 mb-4">
            <CustomCheckbox
              checked={ctrl.poll.response_editable}
              onChange={() =>
                ctrl.onChangeResponseEditable(!ctrl.poll.response_editable)
              }
              data-pw="response-editable-checkbox"
            />
            <div className="flex flex-col gap-1">
              <label className="text-sm font-medium text-white cursor-pointer">
                {t('response_editable_label')}
              </label>
              <p className="text-xs text-neutral-400">
                {t('response_editable_description')}
              </p>
            </div>
          </div>
        )}

        <Card>
          <Col>
            {ctrl.space.isAdmin() && ctrl.poll.user_response_count == 0 && (
              <Row className="gap-2 justify-end mb-4">
                {ctrl.editing.get() ? (
                  <>
                    <Button variant="primary" onClick={ctrl.handleSave}>
                      {t('btn_save')}
                    </Button>
                    <Button onClick={ctrl.handleDiscard}>
                      {t('btn_discard')}
                    </Button>
                  </>
                ) : (
                  <Button onClick={ctrl.handleEdit}>{t('btn_edit')}</Button>
                )}
              </Row>
            )}

            {ctrl.editing.get() ? (
              <SurveyEditor ctrl={ctrl} />
            ) : (
              <SurveyViewer
                t={ctrl.t}
                status={ctrl.poll.status}
                isAdmin={true}
                isLogin={!!ctrl.user}
                canSubmit={false}
                questions={ctrl.poll.questions}
                onUpdateAnswer={ctrl.handleUpdateAnswer}
                selectedAnswers={ctrl.answers.get()}
              />
            )}
          </Col>
        </Card>

        {ctrl.space.spaceType == SpaceType.Deliberation && (
          <div className="flex flex-row w-full justify-end">
            <Button className="w-fit" onClick={ctrl.handleBack}>
              {t('btn_back')}
            </Button>
          </div>
        )}
      </Col>
    </>
  );
}
