import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { SpacePanelResponse } from '../../types/space-panel-response';
import { usePopup } from '@/lib/contexts/popup-service';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useUpdatePanelMutation } from '../../hooks/use-update-panel-mutation';
import { PanelAttribute } from '../../types/panel-attribute';

export class SpacePanelEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: SpacePanelResponse,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpacePanelEditor', undefined>,

    public updatePanel: ReturnType<typeof useUpdatePanelMutation>,
  ) {}

  handleUpdateValues = async (v: PanelAttribute[]) => {
    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      quotas: this.panel.quotas,
      attributes: v,
    });
  };

  handleUpdateQuota = async (quotas: number) => {
    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      quotas,
      attributes: this.panel.attributes,
    });
  };
}

export function useSpacePanelEditorController(spacePk: string) {
  const popup = usePopup();
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);

  const updatePanel = useUpdatePanelMutation();

  return new SpacePanelEditorController(
    spacePk,
    space,
    panel,
    popup,
    t,
    updatePanel,
  );
}
