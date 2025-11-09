import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePollsViewerController } from './use-space-polls-viewer-controller';
import { PollList } from '../../../components/poll-list';

export function SpacePollsViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePollsViewerPage: spacePk=${spacePk}`);
  const ctrl = useSpacePollsViewerController(spacePk);

  return (
    <>
      <Col>
        <PollList
          t={ctrl.t}
          polls={ctrl.polls.get()}
          bookmark={ctrl.bookmark.get()}
          enterPoll={ctrl.enterPoll}
          loadMore={ctrl.loadMore}
        />
      </Col>
    </>
  );
}
