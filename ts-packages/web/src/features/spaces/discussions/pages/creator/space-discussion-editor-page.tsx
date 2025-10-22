import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../space-discussion-path-props';
import { useSpaceDiscussionEditorController } from './use-space-discussion-editor-controller';
import { Col } from '@/components/ui/col';
import DiscussionEditor from '../../components/discussion-editor';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';

export function SpaceDiscussionEditorPage({
  spacePk,
}: SpaceDiscussionPathProps) {
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
            canEdit={true}
            isPublished={!ctrl.space.isDraft}
            discussions={ctrl.discussion.discussions}
            bookmark={ctrl.discussion.bookmark}
          />
        </Col>
      </Col>
    </>
  );
}
