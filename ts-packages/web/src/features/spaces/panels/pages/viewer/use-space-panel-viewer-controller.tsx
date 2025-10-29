import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { ListPanelResponse } from '../../types/list-panel-response';

export class SpacePanelViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: ListPanelResponse,
  ) {}
}

export function useSpacePanelViewerController(spacePk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);

  return new SpacePanelViewerController(spacePk, space, panel);
}
