import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePollsEditorController } from './use-space-polls-editor-controller';
import { PollList } from '../../../components/poll-list';
import { PollTypeSelector } from '../../../components/poll-type-selector';
import { Button } from '@/components/ui/button';

export function SpacePollsEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePollsEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpacePollsEditorController(spacePk);
  const polls = ctrl.polls.get();

  return (
    <>
      <Col crossAxisAlignment="end">
        {polls.length > 0 && (
          <Button
            variant="primary"
            className="w-[120px]"
            onClick={() => {
              logger.debug('Show poll type selector');
              ctrl.showSelector.set(true);
            }}
          >
            {ctrl.t('create_poll')}
          </Button>
        )}

        {polls.length === 0 || ctrl.showSelector.get() ? (
          <PollTypeSelector
            t={ctrl.t}
            onSelectType={ctrl.handleCreatePoll}
            showPrePoll={ctrl.shouldShowPrePoll()}
          />
        ) : (
          <PollList
            t={ctrl.t}
            polls={polls}
            bookmark={ctrl.bookmark.get()}
            enterPoll={ctrl.enterPoll}
            deletePoll={ctrl.handleDeletePoll}
            loadMore={ctrl.loadMore}
          />
        )}
      </Col>
    </>
  );
}
