import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoViewerController } from './space-dao-viewer-controller';

export function SpaceDaoViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoViewerPage: spacePk=${spacePk}`);
  const _ctrl = useSpaceDaoViewerController(spacePk);

  return <div className="w-full">SpaceDaoViewerPage</div>;
}
