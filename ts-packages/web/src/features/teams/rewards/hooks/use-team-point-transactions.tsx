import { useInfiniteQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { ListPointTransactionsResponse } from '../types';

export const QK_TEAM_POINT_TRANSACTIONS = 'team-point-transactions';

export async function listTeamPointTransactions(
  teamPk: string,
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
  const path = `/v3/teams/${teamPk}/points/transactions${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function useTeamPointTransactions(
  teamPk: string,
  month?: string,
  limit: number = 10,
) {
  return useInfiniteQuery({
    queryKey: [QK_TEAM_POINT_TRANSACTIONS, teamPk, month],
    queryFn: async ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPointTransactionsResponse> => {
      return listTeamPointTransactions(teamPk, month, pageParam, limit);
    },
    getNextPageParam: (last: ListPointTransactionsResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
  });
}
