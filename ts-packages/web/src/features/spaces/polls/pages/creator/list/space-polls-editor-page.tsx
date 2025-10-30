import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePollsEditorController } from './use-space-polls-editor-controller';
import { PollList } from '../../../components/poll-list';

export function SpacePollsEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePollsEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpacePollsEditorController(spacePk);

  return (
    <>
      <Col>
        <PollList
          t={ctrl.t}
          polls={ctrl.polls.get()}
          bookmark={ctrl.bookmark.get()}
          canEdit={ctrl.space.isAdmin()}
          createPoll={ctrl.handleCreatePoll}
          enterPoll={ctrl.enterPoll}
          loadMore={ctrl.loadMore}
        />
      </Col>
    </>
  );
}
