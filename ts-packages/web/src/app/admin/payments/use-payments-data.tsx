import { useInfiniteQuery } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { adminKeys } from '@/constants';
import type { AdminPaymentListResponse } from '@/features/admin/types/admin-user';

export async function listPayments(
  bookmark?: string,
): Promise<AdminPaymentListResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }

  const queryString = params.toString();
  const path = `/m3/payments${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function getOptions() {
  return {
    queryKey: adminKeys.payments(),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<AdminPaymentListResponse> => listPayments(pageParam),
    getNextPageParam: (last: AdminPaymentListResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function usePaymentsData() {
  return useInfiniteQuery(getOptions());
}
