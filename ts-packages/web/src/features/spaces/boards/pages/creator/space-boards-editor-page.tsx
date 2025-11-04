import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsEditorController } from './space-boards-editor-controller';

export function SpaceBoardsEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsEditorPage: spacePk=${spacePk}`);
  const _ctrl = useSpaceBoardsEditorController(spacePk);

  return (
    <>
      <div>space boards editor page</div>
    </>
  );
}
