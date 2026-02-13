import { useMutation, useQueryClient } from '@tanstack/react-query';
import { cancelPayment } from '@/lib/api/ratel/admin.m3';
import { adminKeys } from '@/constants';
import type { AdminCancelPaymentRequest } from '@/features/admin/types/admin-user';

export function useCancelPaymentMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['cancel-payment'],
    mutationFn: ({
      paymentId,
      request,
    }: {
      paymentId: string;
      request: AdminCancelPaymentRequest;
    }) => cancelPayment(paymentId, request),
    onSettled: async () => {
      await qc.invalidateQueries({
        queryKey: adminKeys.payments(),
      });
    },
  });
}
