import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Attribute, PanelAttribute } from '../types/panel-attribute';
import { updateSpacePanelQuotas } from '@/lib/api/ratel/panel.spaces.v3';

type Vars = {
  spacePk: string;
  quotas: number;
  attribute: PanelAttribute;
  value: Attribute;
};

export function useUpdatePanelQuotaMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-panel-quotas'],
    mutationFn: async ({ spacePk, quotas, attribute, value }: Vars) => {
      await updateSpacePanelQuotas(spacePk, quotas, attribute, value);
    },
    onSuccess: async (_data, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
