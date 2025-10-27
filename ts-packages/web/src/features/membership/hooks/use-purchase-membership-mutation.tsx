import { QK_MY_MEMBERSHIPS } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { PurchaseMembershipRequest } from '../dto/purchase-membership-request';

export function usePurchaseMembershipMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: PurchaseMembershipRequest) =>
      call('POST', '/v3/memberships', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_MY_MEMBERSHIPS] });
    },
  });
}
