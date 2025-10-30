import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { deleteSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';
import { ListPanelResponse } from '../types/list-panel-response';

export function useDeletePanelMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-panel'],
    mutationFn: async ({
      spacePk,
      panelPk,
    }: {
      spacePk: string;
      panelPk: string;
    }) => {
      await deleteSpacePanel(spacePk, panelPk);
      return { spacePk, panelPk };
    },

    onMutate: async ({ spacePk, panelPk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListPanelResponse>(qk);

      qc.setQueryData<ListPanelResponse>(qk, (old) => {
        if (!old) return old;
        return new ListPanelResponse({
          panels: old.panels.filter((d) => d.pk !== panelPk),
          bookmark: old.bookmark,
        });
      });

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
