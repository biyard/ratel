import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerController } from './space-boards-viewer-controller';

export function SpaceBoardsViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsEditorPage: spacePk=${spacePk}`);
  const _ctrl = useSpaceBoardsViewerController(spacePk);

  return (
    <>
      <div>space boards viewer page</div>
    </>
  );
}
