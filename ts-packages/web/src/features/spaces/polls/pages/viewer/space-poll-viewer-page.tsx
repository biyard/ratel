import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import { Col } from '@/components/ui/col';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import { TimeRangeSetting } from '../../components/time-range-setting';
import Card from '@/components/card';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';

export function SpacePollViewerPage({ spacePk, pollPk }: SpacePollPathProps) {
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
