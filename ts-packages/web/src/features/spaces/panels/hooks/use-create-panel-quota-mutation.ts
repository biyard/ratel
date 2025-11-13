import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { PanelAttributeWithQuota } from '../types/panel-attribute';
import { call } from '@/lib/api/ratel/call';
import { SpacePanel } from '../types/space-panel';

type Vars = {
  attributes: PanelAttributeWithQuota[];
};

export function useCreatePanelQuotaMutation(spacePk: string) {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-panel-quotas'],
    mutationFn: async ({ attributes }: Vars) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const res: any = await call(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels`,
        {
          attributes,
        },
      );

      return { panels: res.map((e) => new SpacePanel(e)) };
    },
    onSuccess: async (_data, _vars, _ctx) => {
      const queryKey = spaceKeys.panels(spacePk);
      qc.invalidateQueries({ queryKey });
    },
  });
}
