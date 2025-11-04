import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsCreateController } from './space-boards-create-controller';

export function SpaceBoardsCreatePage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsCreatePage: spacePk=${spacePk}`);
  const _ctrl = useSpaceBoardsCreateController(spacePk);

  return (
    <>
      <div>space boards create page</div>
    </>
  );
}
