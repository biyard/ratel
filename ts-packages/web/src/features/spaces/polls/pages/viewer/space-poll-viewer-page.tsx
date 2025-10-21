import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import { Col } from '@/components/ui/col';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import { TimeRangeSetting } from '../../components/time-range-setting';
import Card from '@/components/card';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';

// TODO: Add to 'i18n/config.ts'
export const i18nSpacePollViewerPage = {
  en: {
    btn_submit: 'Submit',
    btn_login: 'Log in to Submit',
  },
  ko: {
    btn_submit: '제출',
    btn_login: '로그인 후 제출',
  },
};

export function SpacePollViewerPage({ spacePk, pollPk }: SpacePollPathProps) {
  // TODO: use or define hooks
  logger.debug(`SpacePollViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`);

  const ctrl = useSpacePollViewerController(spacePk, pollPk);

  return (
    <>
      <Col>
        <TimeRangeSetting
          startTimestampMillis={ctrl.poll.started_at}
          endTimestampMillis={ctrl.poll.ended_at}
          className="justify-end"
        />

        <Card>
          <Col>
            <SurveyViewer
              t={ctrl.t}
              questions={ctrl.poll.questions}
              onUpdateAnswer={ctrl.handleUpdateAnswer}
              selectedAnswers={ctrl.answers.get()}
            />
          </Col>
        </Card>

        <Row className="justify-end w-full">
          {ctrl.user ? (
            <Button onClick={ctrl.handleSubmit}>
              {ctrl.t('SpacePollViewer:btn_submit')}
            </Button>
          ) : (
            <Button onClick={ctrl.handleLogin}>
              {ctrl.t('SpacePollViewer:btn_login')}
            </Button>
          )}
        </Row>
      </Col>
    </>
  );
}
