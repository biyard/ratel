import { logger } from '@/lib/logger';
import { useSpaceDiscussionViewerController } from './use-space-discussion-viewer-controller';
import { Col } from '@/components/ui/col';
import DiscussionEditor from '../../components/discussion-editor';
import { SpacePathProps } from '@/features/space-path-props';

export function SpaceDiscussionViewerPage({ spacePk }: SpacePathProps) {
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
