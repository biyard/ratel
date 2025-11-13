import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { usePopup } from '@/lib/contexts/popup-service';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useUpdatePanelMutation } from '../../hooks/use-update-panel-mutation';
import {
  PanelAttribute,
  PanelAttributeWithQuota,
} from '../../types/panel-attribute';
import { useCreatePanelQuotaMutation } from '../../hooks/use-create-panel-quota-mutation';
import { useDeletePanelQuotaMutation } from '../../hooks/use-delete-panel-quota-mutation';
import { useUpdatePanelQuotaMutation } from '../../hooks/use-update-panel-quota-mutation';
import {
  convertOptionsToPanelAttributes,
  getAllPanelAttributeOptions,
  getAttributeWithDefaultQuotas,
  PanelAttributeOptions,
} from './panel-attribute-options';
import { useMemo, useState } from 'react';
import { State } from '@/types/state';
import { useListPanels } from '../../hooks/use-list-panels';

export class SpacePanelEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpacePanelEditor', undefined>,

    public updatePanel: ReturnType<typeof useUpdatePanelMutation>,
    public createPanelQuota: ReturnType<typeof useCreatePanelQuotaMutation>,
    public deletePanelQuota: ReturnType<typeof useDeletePanelQuotaMutation>,
    public updatePanelQuota: ReturnType<typeof useUpdatePanelQuotaMutation>,
    public selectedAttribute: PanelAttributeOptions[],
    public attributes: State<PanelAttributeWithQuota[]>,
    public panels: ReturnType<typeof useListPanels>['data'],
  ) {}

  handleUpdateAttributeQuota = async (sk: string, quota: number) => {
    await this.updatePanelQuota.mutateAsync({
      sk,
      quota,
    });
  };

  handleDeleteAttributeQuota = async (pk: string, sk: string) => {
    this.deletePanelQuota.mutate({
      keys: [
        {
          pk,
          sk,
        },
      ],
    });
  };

  handleChangeSelectedAttributes = (attrs: PanelAttributeOptions[]) => {
    const removed = this.selectedAttribute.filter((e) => !attrs.includes(e));
    const added = attrs.filter((e) => !this.selectedAttribute.includes(e));

    if (removed.length > 0) {
      const panels = this.panels.filter((p) => {
        return removed.some((r) => p.isOption(r));
      });

      this.deletePanelQuota.mutateAsync({
        keys: panels.map((attr) => {
          return {
            pk: attr.pk,
            sk: attr.sk,
          };
        }),
      });
    } else if (added.length > 0) {
      const defaultAttrs = convertOptionsToPanelAttributes(added);

      const attributes = getAttributeWithDefaultQuotas(
        this.space.quota,
        defaultAttrs,
      );

      this.createPanelQuota.mutateAsync({ attributes });
    }
  };

  handleUpdateValues = async (v: PanelAttribute[]) => {
    // TODO
  };

  handleUpdateQuota = async (quotas: number) => {
    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      quota: quotas,
    });
  };

  get allOptions() {
    return getAllPanelAttributeOptions().map((e) => {
      return {
        label: this.t(e),
        value: e,
      };
    });
  }
}

export function useSpacePanelEditorController(spacePk: string) {
  const popup = usePopup();
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);

  const updatePanel = useUpdatePanelMutation();
  const createPanelQuota = useCreatePanelQuotaMutation(spacePk);
  const deletePanelQuota = useDeletePanelQuotaMutation(spacePk);
  const updatePanelQuota = useUpdatePanelQuotaMutation(spacePk);
  const panels = useListPanels(spacePk);

  const selectedAttribute = useMemo(() => {
    return Array.from(
      new Set(panels.data.map((e) => e.toPanelOption()).flatMap((e) => e)),
    );
  }, [panels]);

  const attributes = useState([]);

  return new SpacePanelEditorController(
    spacePk,
    space,
    popup,
    t,
    updatePanel,
    createPanelQuota,
    deletePanelQuota,
    updatePanelQuota,
    selectedAttribute,
    new State(attributes),
    panels.data,
  );
}
