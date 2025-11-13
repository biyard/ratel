import { spaceKeys } from '@/constants';
import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpacePanel } from '../types/space-panel';

type Vars = {
  keys: { pk: string; sk: string }[];
};

export function useDeletePanelQuotaMutation(spacePk: string) {
  return useMutation({
    mutationKey: ['delete-panel-quotas'],
    mutationFn: async ({ keys }: Vars) => {
      call('DELETE', `/v3/spaces/${encodeURIComponent(spacePk)}/panels`, {
        keys,
      });
    },
    onMutate: async ({ keys }) => {
      const queryKey = spaceKeys.panels(spacePk);

      const rollback = await optimisticUpdate<SpacePanel[]>(
        { queryKey },
        (old) => {
          if (!old) return old;

          const news = old.filter(
            (p) => !keys.some((k) => k.pk === p.pk && k.sk === p.sk),
          );

          return news;
        },
      );

      return { rollback };
    },
    onError: async (_err, _vars, _context) => {
      if (_context?.rollback) {
        _context.rollback.rollback();
      }
    },
    onSuccess: async (_data, _vars, _ctx) => {},
  });
}
