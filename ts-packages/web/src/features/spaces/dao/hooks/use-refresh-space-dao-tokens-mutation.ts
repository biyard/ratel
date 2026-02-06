import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

type RefreshSpaceDaoTokensResponse = {
  updated: number;
  last_block: number;
};

export function useRefreshSpaceDaoTokensMutation(spacePk: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return call<void, RefreshSpaceDaoTokensResponse>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/tokens/refresh`,
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: spaceDaoKeys.tokens(spacePk),
      });
    },
  });
}
