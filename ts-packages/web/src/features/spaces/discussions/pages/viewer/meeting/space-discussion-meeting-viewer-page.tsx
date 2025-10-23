import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../../space-discussion-path-props';
import { useSpaceDiscussionMeetingViewerController } from './use-space-discussion-meeting-viewer-controller';

export function SpaceDiscussionMeetingViewerPage({
  spacePk,
  discussionPk,
}: SpaceDiscussionPathProps) {
  logger.debug(
    `SpaceDiscussionMeetingViewerPage: spacePk=${spacePk} discussionPk=${discussionPk}`,
  );

  const _ctrl = useSpaceDiscussionMeetingViewerController(
    spacePk,
    discussionPk,
  );

  return (
    <div className="bg-white text-black fixed top-0 left-0 flex flex-row w-full h-full">
      discussion page
    </div>
  );
}
