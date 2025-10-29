import { logger } from '@/lib/logger';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpacePanelEditorController } from './use-space-panel-editor-controller';

export function SpacePanelEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpacePanelEditorPage: spacePk=${spacePk}`);
  const _ctrl = useSpacePanelEditorController(spacePk);

  return (
    <>
      <div className="text-white">panel editor page</div>
    </>
  );
}
