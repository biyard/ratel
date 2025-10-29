import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { ListPanelResponse } from '../../types/list-panel-response';

export class SpacePanelEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: ListPanelResponse,
  ) {}
}

export function useSpacePanelEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);

  return new SpacePanelEditorController(spacePk, space, panel);
}
