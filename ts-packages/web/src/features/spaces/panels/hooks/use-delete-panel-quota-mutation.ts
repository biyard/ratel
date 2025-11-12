import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Attribute, PanelAttribute } from '../types/panel-attribute';
import { deleteSpacePanelQuotas } from '@/lib/api/ratel/panel.spaces.v3';

type Vars = {
  spacePk: string;
  attribute: PanelAttribute;
  value: Attribute;
};

export function useDeletePanelQuotaMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-panel-quotas'],
    mutationFn: async ({ spacePk, attribute, value }: Vars) => {
      await deleteSpacePanelQuotas(spacePk, attribute, value);
    },
    onSuccess: async (_data, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
