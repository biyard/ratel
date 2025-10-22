import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../space-discussion-path-props';
import { useSpaceDiscussionEditorController } from './use-space-discussion-editor-controller';
import { Col } from '@/components/ui/col';
import DiscussionEditor from '../../components/discussion-editor';

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
            canEdit={true}
          />
        </Col>
      </Col>
    </>
  );
}
