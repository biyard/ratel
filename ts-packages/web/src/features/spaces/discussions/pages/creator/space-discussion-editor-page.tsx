import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../space-discussion-path-props';
import { useSpaceDiscussionEditorController } from './use-space-discussion-editor-controller';

export function SpaceDiscussionEditorPage({
  spacePk,
}: SpaceDiscussionPathProps) {
  logger.debug(`SpaceDiscussionEditorPage: spacePk=${spacePk}`);

  const _ctrl = useSpaceDiscussionEditorController(spacePk);

  return (
    <>
      <div className="text-text-primary">discussion editor</div>
    </>
  );
}
