import { logger } from '@/lib/logger';
import { useSpacePollViewerController } from './use-space-poll-viewer-controller';
import { SpacePollPathProps } from '../space-poll-path-props';

export function SpacePollViewerPage({ spacePk, pollPk }: SpacePollPathProps) {
  // TODO: use or define hooks
  logger.debug(`SpacePollViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`);

  const _ctrl = useSpacePollViewerController();

  return (
    <>
      <div>SpacePollViewerPage</div>
    </>
  );
}
