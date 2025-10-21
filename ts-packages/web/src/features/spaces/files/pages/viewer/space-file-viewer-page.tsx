import { logger } from '@/lib/logger';
import { SpaceFilePathProps } from '../space-file-path-props';
import { useSpaceFileViewerController } from './use-space-file-viewer-controller';
import SpaceFileViewer from '@/features/spaces/components/file/viewer';

export function SpaceFileViewerPage({ spacePk }: SpaceFilePathProps) {
  logger.debug(`SpacePollViewerPage: spacePk=${spacePk}`);

  const ctrl = useSpaceFileViewerController(spacePk);

  return (
    <>
      <SpaceFileViewer files={ctrl.files} />
    </>
  );
}
