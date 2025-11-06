import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Attribute } from '../types/answer-type';
import { updateSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';
import { ListPanelResponse } from '../types/list-panel-response';
import { SpacePanelResponse } from '../types/space-panel-response';

type Vars = {
  spacePk: string;
  panelPk: string;
  name: string;
  quotas: number;
  attributes: Attribute[];
};

export function useUpdatePanelMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-panel'],
    mutationFn: async (v: Vars) => {
      const { spacePk, panelPk, name, quotas, attributes } = v;

      await updateSpacePanel(spacePk, panelPk, name, quotas, attributes);

      return v;
    },

    onMutate: async (vars) => {
      const { spacePk, panelPk, ...patch } = vars;

      const qk = spaceKeys.panels(spacePk);
      await qc.cancelQueries({ queryKey: qk });

      const prev = qc.getQueryData<ListPanelResponse>(qk);

      qc.setQueryData<ListPanelResponse>(qk, (old) => {
        if (!old) return old;

        const updatedList = old.panels.map((d): SpacePanelResponse => {
          if (d.pk !== panelPk) return d;

          return {
            ...d,
            name: patch.name,
            quotas: patch.quotas,
            attributes: patch.attributes,
          } as SpacePanelResponse;
        });

        return new ListPanelResponse({
          panels: updatedList,
          bookmark: old.bookmark,
        });
      });

      return { qk, prev };
    },

    onError: (_err, _vars, ctx) => {
      if (ctx?.qk) qc.setQueryData(ctx.qk, ctx.prev);
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
