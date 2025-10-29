import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelViewerController } from './use-space-panel-viewer-controller';

export function SpacePanelViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelViewerPage: spacePk=${spacePk}`);
  const _ctrl = useSpacePanelViewerController(spacePk);

  return (
    <>
      <div className="text-white">panel viewer page</div>
    </>
  );
}
