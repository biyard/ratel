import { logger } from '@/lib/logger';
import { useSpaceFileViewerController } from './use-space-file-viewer-controller';
import SpaceFileViewer from '../../components/space-file-viewer';
import { SpacePathProps } from '@/features/space-path-props';

export function SpaceFileViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceFileViewerPage: spacePk=${spacePk}`);

  const ctrl = useSpaceFileViewerController(spacePk);

  return (
    <>
      <SpaceFileViewer files={ctrl.files} />
    </>
  );
}
