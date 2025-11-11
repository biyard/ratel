import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { participateSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';

export function useParticipatePanelMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['participate-panel'],
    mutationFn: async ({
      spacePk,
      panelPk,
    }: {
      spacePk: string;
      panelPk: string;
    }) => {
      await participateSpacePanel(spacePk, panelPk);
      return { spacePk, panelPk };
    },
    onSuccess: async (_data, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
