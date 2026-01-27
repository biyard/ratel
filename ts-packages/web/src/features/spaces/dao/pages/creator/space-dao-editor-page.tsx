import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoEditorController } from './space-dao-editor-controller';

export function SpaceDaoEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoEditorPage: spacePk=${spacePk}`);
  const _ctrl = useSpaceDaoEditorController(spacePk);

  return <div className="w-full">SpaceDaoEditorPage</div>;
}
