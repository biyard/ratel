import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { ListPanelResponse } from '../../types/list-panel-response';
import { useCreatePanelMutation } from '../../hooks/use-create-panel-mutation';
import { useEffect, useState } from 'react';
import { State } from '@/types/state';
import { SpacePanelResponse } from '../../types/space-panel-response';
import { useUpdatePanelMutation } from '../../hooks/use-update-panel-mutation';
import { usePopup } from '@/lib/contexts/popup-service';
import AgePopup from '../../components/modals/age_popup';
import { Attribute } from '../../types/answer-type';
import GenderPopup from '../../components/modals/gender_popup';
import { useDeletePanelMutation } from '../../hooks/use-delete-panel-mutation';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';

export class SpacePanelEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: ListPanelResponse,
    public panels: State<SpacePanelResponse[]>,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpacePanelEditor', undefined>,
    public createPanel: ReturnType<typeof useCreatePanelMutation>,
    public updatePanel: ReturnType<typeof useUpdatePanelMutation>,
    public deletePanel: ReturnType<typeof useDeletePanelMutation>,
  ) {}

  handleAddPanel = async () => {
    await this.createPanel.mutateAsync({
      spacePk: this.spacePk,
      name: '',
      quotas: 0,
      attributes: [],
    });
  };

  handleUpdateName = async (index: number, name: string) => {
    const panel = this.panels.get()[index];
    panel.name = name;
    const panelPk = panel.pk;

    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      panelPk,
      name,
      quotas: panel.quotas,
      attributes: panel.attributes,
    });
  };

  handleUpdateQuotas = async (index: number, quotas: number) => {
    const panel = this.panels.get()[index];
    panel.quotas = quotas;
    const panelPk = panel.pk;

    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      panelPk,
      name: panel.name,
      quotas,
      attributes: panel.attributes,
    });
  };

  openGenderPopup = (index: number) => {
    const panel = this.panels.get()[index];
    this.popup
      .open(
        <GenderPopup
          t={this.t}
          attributes={panel.attributes}
          onSave={async (attributes: Attribute[]) => {
            const panel = this.panels.get()[index];
            panel.attributes = attributes;
            const panelPk = panel.pk;

            await this.updatePanel.mutateAsync({
              spacePk: this.spacePk,
              panelPk,
              name: panel.name,
              quotas: panel.quotas,
              attributes,
            });
            this.popup.close();
          }}
          onClose={() => {
            this.popup.close();
          }}
        />,
      )
      .withTitle(this.t('gender_modal_title'));
  };

  openAgePopup = (index: number) => {
    const panel = this.panels.get()[index];
    this.popup
      .open(
        <AgePopup
          attributes={panel.attributes}
          t={this.t}
          onSave={async (attributes: Attribute[]) => {
            const panel = this.panels.get()[index];
            panel.attributes = attributes;
            const panelPk = panel.pk;

            await this.updatePanel.mutateAsync({
              spacePk: this.spacePk,
              panelPk,
              name: panel.name,
              quotas: panel.quotas,
              attributes,
            });
            this.popup.close();
          }}
          onClose={() => {
            this.popup.close();
          }}
        />,
      )
      .withTitle(this.t('age_modal_title'));
  };

  handleDeletePanel = async (index: number) => {
    const panel = this.panels.get()[index];
    const panelPk = panel.pk;

    await this.deletePanel.mutateAsync({
      spacePk: this.spacePk,
      panelPk,
    });
  };
}

export function useSpacePanelEditorController(spacePk: string) {
  const popup = usePopup();
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);
  const panels = useState(panel.panels || []);

  useEffect(() => {
    panels[1](panel.panels || []);
  }, [panel.panels]);

  const createPanel = useCreatePanelMutation();
  const updatePanel = useUpdatePanelMutation();
  const deletePanel = useDeletePanelMutation();

  return new SpacePanelEditorController(
    spacePk,
    space,
    panel,
    new State(panels),
    popup,
    t,
    createPanel,
    updatePanel,
    deletePanel,
  );
}
