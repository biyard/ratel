import { useQuery } from '@tanstack/react-query';
import { listPayments } from '@/lib/api/ratel/admin.m3';
import type {
  AdminPaymentListResponse,
  PaymentBookmark,
} from '@/features/admin/types/admin-user';

const PAGE_SIZE = 10;

function parseBookmark(bookmarkStr: string | null): PaymentBookmark | null {
  if (!bookmarkStr) return null;
  try {
    return JSON.parse(bookmarkStr);
  } catch {
    return null;
  }
}

export function usePaymentsData(page: number) {
  return useQuery<{
    payments: AdminPaymentListResponse['items'];
    totalCount: number;
    totalPages: number;
  }>({
    queryKey: ['admin-payments', page],
    queryFn: async () => {
      const response = await listPayments(page);
      const bookmark = parseBookmark(response.bookmark);
      return {
        payments: response.items,
        totalCount: bookmark?.total_count ?? response.items.length,
        totalPages:
          bookmark?.total_pages ??
          Math.ceil(
            (bookmark?.total_count ?? response.items.length) / PAGE_SIZE,
          ),
      };
    },
  });
}
