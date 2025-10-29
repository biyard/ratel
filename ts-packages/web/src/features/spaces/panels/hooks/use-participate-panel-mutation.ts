import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { participateSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';
import { ListPanelResponse } from '../types/list-panel-response';

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

    onMutate: async ({ spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListPanelResponse>(qk);

      return { qk, prev };
    },

    onError: (_err, _vars, ctx) => {
      if (ctx?.qk && ctx?.prev) {
        qc.setQueryData(ctx.qk, ctx.prev);
      }
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
