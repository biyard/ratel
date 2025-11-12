import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Attribute, PanelAttribute } from '../types/panel-attribute';
import { createSpacePanelQuotas } from '@/lib/api/ratel/panel.spaces.v3';

type Vars = {
  spacePk: string;
  quotas: number[];
  attributes: PanelAttribute[];
  values: Attribute[];
};

export function useCreatePanelQuotaMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-panel-quotas'],
    mutationFn: async ({ spacePk, quotas, attributes, values }: Vars) => {
      await createSpacePanelQuotas(spacePk, quotas, attributes, values);
    },
    onSuccess: async (_data, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
