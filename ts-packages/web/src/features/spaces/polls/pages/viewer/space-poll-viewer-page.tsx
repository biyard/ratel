import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import { Col } from '@/components/ui/col';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import Card from '@/components/card';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';
import { SpaceType } from '@/features/spaces/types/space-type';
import { TimeRangeDisplay } from '@/features/spaces/boards/components/time-range-display';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { useMemo } from 'react';

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

  const canSubmit = useMemo(() => {
    return (
      ctrl.user &&
      ctrl.poll.myResponse.length === 0 &&
      (!ctrl.space.anonymous_participation || ctrl.space.participated) &&
      ctrl.space.status !== SpaceStatus.Finished
    );
  }, [ctrl]);

  const canUpdate =
    ctrl.user &&
    ctrl.poll.myResponse.length > 0 &&
    ctrl.poll.response_editable &&
    ctrl.space.status !== SpaceStatus.Finished;

  return (
    <>
      <Col>
        <TimeRangeDisplay
          startTimestampMillis={ctrl.poll.started_at}
          endTimestampMillis={ctrl.poll.ended_at}
        />

        <Card>
          <Col>
            <SurveyViewer
              t={ctrl.t}
              questions={ctrl.poll.questions}
              status={ctrl.poll.status}
              onUpdateAnswer={ctrl.handleUpdateAnswer}
              selectedAnswers={ctrl.answers.get()}
              onSubmit={ctrl.handleSubmit}
              onLogin={ctrl.handleLogin}
              canParticipate={
                (ctrl.space.isAdmin() ||
                  ctrl.space.spaceType !== SpaceType.Deliberation ||
                  ctrl.space.participated) &&
                ctrl.space.status !== SpaceStatus.Started
              }
              canSubmit={canSubmit}
              disabled={!canSubmit && !canUpdate}
              canUpdate={canUpdate}
              isLogin={!!ctrl.user}
              isFinished={
                ctrl.space.status === SpaceStatus.Finished ||
                ctrl.poll.ended_at < Date.now()
              }
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
