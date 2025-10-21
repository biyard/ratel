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
    btn_update: 'Update',
    btn_login: 'Log in to Submit',
  },
  ko: {
    btn_submit: '제출',
    btn_update: '수정',
    btn_login: '로그인 후 제출',
  },
};

export function SpacePollViewerPage({ spacePk, pollPk }: SpacePollPathProps) {
  // TODO: use or define hooks
  logger.debug(`SpacePollViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`);

  const ctrl = useSpacePollViewerController(spacePk, pollPk);

  let button = <></>;

  if (ctrl.user && ctrl.poll.myResponse.length === 0) {
    button = (
      <Button onClick={ctrl.handleSubmit}>
        {ctrl.t('SpacePollViewer:btn_submit')}
      </Button>
    );
  } else if (
    ctrl.user &&
    ctrl.poll.myResponse.length > 0 &&
    ctrl.poll.response_editable
  ) {
    button = (
      <Button onClick={ctrl.handleSubmit}>
        {ctrl.t('SpacePollViewer:btn_update')}
      </Button>
    );
  } else if (!ctrl.user) {
    button = (
      <Button onClick={ctrl.handleLogin}>
        {ctrl.t('SpacePollViewer:btn_login')}
      </Button>
    );
  }

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

        <Row className="justify-end w-full">{button}</Row>
      </Col>
    </>
  );
}
