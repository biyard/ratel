import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { logger } from '@/lib/logger';
import { SpaceDiscussionMeetingViewerPage } from '@/features/spaces/discussions/pages/viewer/meeting/space-discussion-meeting-viewer-page';

export default function DiscussionPage() {
  const { spacePk, discussionPk } = useParams<{
    spacePk: string;
    discussionPk: string;
  }>();
  const { data: space } = useSpaceById(spacePk);

  logger.debug('space pk, discussion pk: ', spacePk, discussionPk);

  if (!space) {
    throw new Error('Space not found');
  }

  return (
    <SpaceDiscussionMeetingViewerPage
      spacePk={spacePk}
      discussionPk={discussionPk}
    />
  );
}
