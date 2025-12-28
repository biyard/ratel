import { useInfiniteQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { ListPointTransactionsResponse } from '../types';

export const QK_MY_POINT_TRANSACTIONS = 'my-point-transactions';

export async function listMyPointTransactions(
  bookmark?: string,
  limit?: number,
): Promise<ListPointTransactionsResponse> {
  const params = new URLSearchParams();
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

export function useMyPointTransactions(limit: number = 10) {
  return useInfiniteQuery({
    queryKey: [QK_MY_POINT_TRANSACTIONS],
    queryFn: async ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPointTransactionsResponse> => {
      return listMyPointTransactions(pageParam, limit);
    },
    getNextPageParam: (last: ListPointTransactionsResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
  });
}
