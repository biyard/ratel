import { useInfiniteQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { ListPointTransactionsResponse } from '../types';
import { userKeys } from '@/constants';

export async function listMyPointTransactions(
  month?: string,
  bookmark?: string,
  limit?: number,
): Promise<ListPointTransactionsResponse> {
  const params = new URLSearchParams();
  if (month) {
    params.append('month', month);
  }
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  if (limit) {
    params.append('size', limit.toString());
  }

  const queryString = params.toString();
  const path = `/v3/me/points/transactions${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function useMyPointTransactions(month?: string, limit: number = 10) {
  return useInfiniteQuery({
    queryKey: [...userKeys.reward_lists(month)],
    queryFn: async ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPointTransactionsResponse> => {
      return listMyPointTransactions(month, pageParam, limit);
    },
    getNextPageParam: (last: ListPointTransactionsResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
  });
}
