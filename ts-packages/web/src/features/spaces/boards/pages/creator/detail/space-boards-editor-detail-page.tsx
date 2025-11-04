import { logger } from '@/lib/logger';
import { useSpaceBoardsEditorDetailController } from './space-boards-editor-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';

export function SpaceBoardsEditorDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsEditorDetailPage: spacePk=${spacePk} postPk: ${postPk}`,
  );
  const _ctrl = useSpaceBoardsEditorDetailController(spacePk, postPk);

  return (
    <>
      <div>space boards editor detail page</div>
    </>
  );
}
