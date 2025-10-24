import { logger } from '@/lib/logger';
import { useSpaceDiscussionEditorController } from './use-space-discussion-editor-controller';
import { Col } from '@/components/ui/col';
import DiscussionEditor from '../../components/discussion-editor';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import { SpacePathProps } from '@/features/space-path-props';

export function SpaceDiscussionEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDiscussionEditorPage: spacePk=${spacePk}`);

  const ctrl = useSpaceDiscussionEditorController(spacePk);

  return (
    <>
      <Col>
        <Col>
          <DiscussionEditor
            t={ctrl.t}
            onadd={ctrl.handleAddDiscussion}
            onupdate={async (
              discussionPk: string,
              discussion: SpaceDiscussionResponse,
            ) => {
              await ctrl.handleUpdateDiscussion(discussionPk, discussion);
            }}
            ondelete={async (discussionPk: string) => {
              await ctrl.handleDeleteDiscussion(discussionPk);
            }}
            onenter={async (discussionPk: string) => {
              await ctrl.enterDiscussionRoom(discussionPk);
            }}
            canEdit={true}
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
