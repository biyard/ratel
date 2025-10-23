import { QK_MEMBERSHIPS } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { CreateMembershipRequest } from '../dto/create-membership-request';
import { call } from '@/lib/api/ratel/call';

/**
 * Create a new membership (Admin only)
 */
export function useCreateMembershipMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: CreateMembershipRequest) =>
      call('POST', '/m3/memberships', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_MEMBERSHIPS] });
    },
  });
}
