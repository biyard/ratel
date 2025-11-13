import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';

type Vars = {
  sk: string;
  quota: number;
};

export function useUpdatePanelQuotaMutation(spacePk: string) {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-panel-quotas'],
    mutationFn: async ({ sk, quota }: Vars) => {
      call(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/panels/${encodeURIComponent(sk)}`,
        {
          quota,
        },
      );
    },
    onSuccess: async (_data, _ctx) => {
      const qk = spaceKeys.panels(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
