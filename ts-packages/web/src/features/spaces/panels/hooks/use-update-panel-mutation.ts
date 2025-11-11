import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';
import { PanelAttribute } from '../types/panel-attribute';

type Vars = {
  spacePk: string;
  quotas: number;
  attributes: PanelAttribute[];
};

export function useUpdatePanelMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-panel'],
    mutationFn: async (v: Vars) => {
      const { spacePk, quotas, attributes } = v;

      await updateSpacePanel(spacePk, quotas, attributes);

      return v;
    },

    onSuccess: async (_, { spacePk }) => {
      // Invalidate space details to refetch with updated participant status
      const panelQk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: panelQk });
    },
  });
}
