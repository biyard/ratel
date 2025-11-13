import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { PanelAttribute } from '../types/panel-attribute';
import { updateSpacePanelQuotas } from '@/lib/api/ratel/panel.spaces.v3';

type Vars = {
  spacePk: string;
  quotas: number;
  attribute: PanelAttribute;
};

export function useUpdatePanelQuotaMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-panel-quotas'],
    mutationFn: async ({ spacePk, quotas, attribute }: Vars) => {
      await updateSpacePanelQuotas(spacePk, quotas, attribute);
    },
    onSuccess: async (_data, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
