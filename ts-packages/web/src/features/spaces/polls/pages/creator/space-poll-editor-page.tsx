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

export function SpacePollEditorPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollEditorPage: spacePk=${spacePk}, pollPk=${pollPk}`);
  const ctrl = useSpacePollEditorController(spacePk, pollPk);
  const { t } = useTranslation('SpacePollEditor');

  return (
    <>
      <Col>
        <Row className="gap-2 justify-end mb-4">
          {ctrl.editing.get() ? (
            <>
              <Button variant="primary" onClick={ctrl.handleSave}>
                {t('btn_save')}
              </Button>
              <Button onClick={ctrl.handleDiscard}>{t('btn_discard')}</Button>
            </>
          ) : (
            <Button onClick={ctrl.handleEdit}>{t('btn_edit')}</Button>
          )}
        </Row>

        <TimeRangeSetting
          canEdit={ctrl.editing.get()}
          onChange={ctrl.onChangeTimeRange}
          startTimestampMillis={ctrl.poll.started_at}
          endTimestampMillis={ctrl.poll.ended_at}
          className="justify-end"
        />

        {ctrl.editing.get() ? (
          <SurveyEditor ctrl={ctrl} />
        ) : (
          <SurveyViewer
            t={ctrl.t}
            questions={ctrl.poll.questions}
            onUpdateAnswer={ctrl.handleUpdateAnswer}
            selectedAnswers={ctrl.answers.get()}
          />
        )}
      </Col>
    </>
  );
}
