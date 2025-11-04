import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerDetailController } from './space-boards-viewer-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';

export function SpaceBoardsViewerDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsViewerDetailPage: spacePk=${spacePk} postPk=${postPk}`,
  );
  const _ctrl = useSpaceBoardsViewerDetailController(spacePk, postPk);

  return (
    <>
      <div>space boards viewer detail page</div>
    </>
  );
}
