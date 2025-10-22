import { logger } from '@/lib/logger';
import { SpaceDiscussionPathProps } from '../space-discussion-path-props';
import { useSpaceDiscussionViewerController } from './use-space-discussion-viewer-controller';

export function SpaceDiscussionViewerPage({
  spacePk,
}: SpaceDiscussionPathProps) {
  logger.debug(`SpaceDiscussionViewerPage: spacePk=${spacePk}`);

  const _ctrl = useSpaceDiscussionViewerController(spacePk);

  return (
    <>
      <div className="text-text-primary">discussion viewer</div>
    </>
  );
}
