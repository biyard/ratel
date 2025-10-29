import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { ListPanelResponse } from '../../types/list-panel-response';
import { useTranslation } from 'react-i18next';
import { SpacePanelResponse } from '../../types/space-panel-response';
import { State } from '@/types/state';
import { TFunction } from 'i18next';
import { useEffect, useState } from 'react';
import { call } from '@/lib/api/ratel/call';

export class SpacePanelViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: ListPanelResponse,
    public bookmark: State<string | null | undefined>,
    public panels: State<SpacePanelResponse[]>,
    public t: TFunction<'SpacePanelEditor', undefined>,
  ) {}

  loadMore = async () => {
    const bm = this.bookmark.get();
    if (!bm) return;

    const next = await call(
      'GET',
      `/v3/spaces/${encodeURIComponent(this.spacePk)}/panels?bookmark=${encodeURIComponent(
        bm,
      )}`,
    );

    const page = new ListPanelResponse(next);
    const prev = this.panels.get() ?? [];
    this.panels.set([...prev, ...page.panels]);
    this.bookmark.set(page.bookmark ?? null);
  };
}

export function useSpacePanelViewerController(spacePk: string) {
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);
  const panels = useState(panel.panels || []);
  const bookmark = useState<string | null>(panel.bookmark ?? null);

  useEffect(() => {
    bookmark[1](panel.bookmark ?? null);
    panels[1](panel.panels || []);
  }, [panel.panels, panel.bookmark]);

  return new SpacePanelViewerController(
    spacePk,
    space,
    panel,
    new State(bookmark),
    new State(panels),

    t,
  );
}
