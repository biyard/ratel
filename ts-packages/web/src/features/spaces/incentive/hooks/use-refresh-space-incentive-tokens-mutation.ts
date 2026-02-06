import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceIncentiveKeys } from '@/constants';

type RefreshSpaceIncentiveTokensResponse = {
  updated: number;
  last_block: number;
};

export function useRefreshSpaceIncentiveTokensMutation(spacePk: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return call<void, RefreshSpaceIncentiveTokensResponse>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/tokens/refresh`,
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: spaceIncentiveKeys.tokens(spacePk),
      });
    },
  });
}
