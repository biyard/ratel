import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import { Col } from '@/components/ui/col';
import SurveyViewer from '@/features/spaces/components/survey/viewer';
import { TimeRangeSetting } from '../../components/time-range-setting';
import Card from '@/components/card';

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
      </Col>
    </>
  );
}
