import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import { SpacePathProps } from '@/features/space-path-props';
import { PollList } from '../../../components/poll-list';
import { useSpacePollAnalyzesController } from './use-space-poll-analyzes-controller';

export function SpacePollAnalyzesPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePollAnalyzesPage: spacePk=${spacePk}`);

  const ctrl = useSpacePollAnalyzesController(spacePk);

  return (
    <>
      <Col>
        <PollList
          t={ctrl.t}
          polls={ctrl.polls.get()}
          canEdit={false}
          createPoll={() => {}}
          enterPoll={ctrl.enterPoll}
        />
      </Col>
    </>
  );
}
