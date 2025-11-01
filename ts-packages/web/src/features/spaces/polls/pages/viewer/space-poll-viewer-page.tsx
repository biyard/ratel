import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import { Col } from '@/components/ui/col';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import { TimeRangeSetting } from '../../components/time-range-setting';
import Card from '@/components/card';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';
import { SpaceType } from '@/features/spaces/types/space-type';

export function SpacePollViewerPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`);

  const ctrl = useSpacePollViewerController(spacePk, pollPk);
  // let button = <></>;

  // if (ctrl.user && ctrl.poll.myResponse.length === 0) {
  //   button = (
  //     <Button onClick={ctrl.handleSubmit}>
  //       {ctrl.t('SpacePollViewer:btn_submit')}
  //     </Button>
  //   );
  // } else if (
  //   ctrl.user &&
  //   ctrl.poll.myResponse.length > 0 &&
  //   ctrl.poll.response_editable
  // ) {
  //   button = (
  //     <Button onClick={ctrl.handleSubmit}>
  //       {ctrl.t('SpacePollViewer:btn_update')}
  //     </Button>
  //   );
  // } else if (!ctrl.user) {
  //   button = (
  //     <Button onClick={ctrl.handleLogin}>
  //       {ctrl.t('SpacePollViewer:btn_login')}
  //     </Button>
  //   );
  // }

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
              onSubmit={ctrl.handleSubmit}
              onLogin={ctrl.handleLogin}
              canSubmit={ctrl.user && ctrl.poll.myResponse.length === 0}
              canUpdate={
                ctrl.user &&
                ctrl.poll.myResponse.length > 0 &&
                ctrl.poll.response_editable
              }
              isLogin={!!ctrl.user}
            />
          </Col>
        </Card>

        {ctrl.space.spaceType == SpaceType.Deliberation && (
          <Row className="justify-end w-full">
            <Button className="w-fit" onClick={ctrl.handleBack}>
              {ctrl.t('SpacePollViewer:btn_back')}
            </Button>
          </Row>
        )}
      </Col>
    </>
  );
}
