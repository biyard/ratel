import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { ListPanelResponse } from '../../types/list-panel-response';
import { useTranslation } from 'react-i18next';
import { SpacePanelResponse } from '../../types/space-panel-response';
import { State } from '@/types/state';
import { TFunction } from 'i18next';
import { useState } from 'react';

export class SpacePanelViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: ListPanelResponse,
    public panels: State<SpacePanelResponse[]>,
    public t: TFunction<'SpacePanelEditor', undefined>,
  ) {}
}

export function useSpacePanelViewerController(spacePk: string) {
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);
  const panels = useState(panel.panels || []);

  return new SpacePanelViewerController(
    spacePk,
    space,
    panel,
    new State(panels),
    t,
  );
}
