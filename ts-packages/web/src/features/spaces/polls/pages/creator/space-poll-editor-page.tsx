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
import { ArrowLeft } from 'lucide-react';
import { SpaceStatus } from '@/features/spaces/types/space-common';

export function SpacePollEditorPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollEditorPage: spacePk=${spacePk}, pollPk=${pollPk}`);
  const ctrl = useSpacePollEditorController(spacePk, pollPk);
  const { t } = useTranslation('SpacePollEditor');

  return (
    <>
      <Col>
        {ctrl.space.spaceType == SpaceType.Deliberation && (
          <div className="flex flex-row w-full justify-start">
            <ArrowLeft
              className="flex flex-row w-5 h-5 cursor-pointer"
              onClick={ctrl.handleBack}
            />
          </div>
        )}
        <TimeRangeSetting
          canEdit={ctrl.space.isAdmin()}
          onChange={ctrl.onChangeTimeRange}
          startTimestampMillis={ctrl.poll.started_at}
          endTimestampMillis={ctrl.poll.ended_at}
          className="justify-end"
        />

        {ctrl.space.isAdmin() && ctrl.poll.user_response_count == 0 && (
          <div className="flex flex-row items-center gap-3 mb-4">
            <CustomCheckbox
              checked={ctrl.poll.response_editable}
              onChange={() =>
                ctrl.onChangeResponseEditable(!ctrl.poll.response_editable)
              }
              data-pw="response-editable-checkbox"
            />
            <div className="flex flex-col gap-1">
              <label className="text-sm font-medium text-text-primary cursor-pointer">
                {t('response_editable_label')}
              </label>
              <p className="text-xs text-text-secondary">
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
                    <Button
                      variant="primary"
                      onClick={ctrl.handleSave}
                      data-testid="poll-btn-save"
                    >
                      {t('btn_save')}
                    </Button>
                    <Button
                      onClick={ctrl.handleDiscard}
                      data-testid="poll-btn-discard"
                    >
                      {t('btn_discard')}
                    </Button>
                  </>
                ) : (
                  <Button onClick={ctrl.handleEdit} data-testid="poll-btn-edit">
                    {t('btn_edit')}
                  </Button>
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
                canParticipate={ctrl.space.status !== SpaceStatus.Started}
                questions={ctrl.poll.questions}
                onUpdateAnswer={ctrl.handleUpdateAnswer}
                selectedAnswers={ctrl.answers.get()}
                isFinished={
                  ctrl.space.status === SpaceStatus.Finished ||
                  ctrl.poll.ended_at < Date.now()
                }
              />
            )}
          </Col>
        </Card>
      </Col>
    </>
  );
}
