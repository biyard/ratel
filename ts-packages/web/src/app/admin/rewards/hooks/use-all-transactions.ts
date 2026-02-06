import { useInfiniteQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';

export interface ProjectPointTransactionResponse {
  meta_user_id: string;
  month: string;
  transaction_type: string;
  amount: number;
  target_user_id?: string;
  description?: string;
  created_at: number;
}

export interface ListAllTransactionsResponse {
  items: ProjectPointTransactionResponse[];
  bookmark?: string;
}

export function useAllTransactions(date?: string) {
  return useInfiniteQuery({
    queryKey: ['admin', 'all-transactions', date],
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams();
      if (date) params.set('date', date);
      if (pageParam) params.set('bookmark', pageParam);
      params.set('limit', '20');

      const queryString = params.toString();
      const path = `/m3/rewards/transactions${queryString ? `?${queryString}` : ''}`;

      return call<undefined, ListAllTransactionsResponse>('GET', path);
    },
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage) => lastPage.bookmark,
  });
}
