import { spaceKeys } from '@/constants';
import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { optimisticUpdate } from '@/lib/hook-utils';
import { Space } from '../../types/space';

type Vars = {
  spacePk: string;
  quota: number;
};

export function useUpdatePanelMutation() {
  return useMutation({
    mutationKey: ['update-panel'],
    mutationFn: async (v: Vars) => {
      const { spacePk, quota: quotas } = v;

      await call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
        quotas,
      });

      return v;
    },
    onMutate: async ({ spacePk, quota }) => {
      const queryKey = spaceKeys.detail(spacePk);

      const rollback = await optimisticUpdate<Space>({ queryKey }, (old) => {
        if (!old) return old;
        old.quota = quota;

        return old;
      });

      return { rollback };
    },

    onError: async (_err, _vars, { rollback }) => {
      if (rollback) {
        rollback.rollback();
      }
    },
  });
}
