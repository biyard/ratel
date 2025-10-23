import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../space-discussion-path-props';
import { useSpaceDiscussionViewerController } from './use-space-discussion-viewer-controller';
import { Col } from '@/components/ui/col';
import DiscussionEditor from '../../components/discussion-editor';

export function SpaceDiscussionViewerPage({
  spacePk,
}: SpaceDiscussionPathProps) {
  logger.debug(`SpaceDiscussionViewerPage: spacePk=${spacePk}`);

  const ctrl = useSpaceDiscussionViewerController(spacePk);

  return (
    <>
      <Col>
        <Col>
          <DiscussionEditor
            t={ctrl.t}
            onadd={() => {}}
            onupdate={() => {}}
            ondelete={() => {}}
            onenter={async (discussionPk: string) => {
              await ctrl.enterDiscussionRoom(discussionPk);
            }}
            canEdit={false}
            isPublished={!ctrl.space.isDraft}
            discussions={ctrl.discussions.get()}
            bookmark={ctrl.bookmark.get()}
            onloadmore={ctrl.loadMore}
          />
        </Col>
      </Col>
    </>
  );
}
