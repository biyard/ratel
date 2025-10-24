import { useMutation, useQueryClient } from '@tanstack/react-query';
import { UpdateMembershipRequest } from '../dto/update-membership-request';
import { QK_MEMBERSHIPS } from '@/constants';
import { call } from '@/lib/api/ratel/call';

/**
 * Update an existing membership (Admin only)
 */
export function useUpdateMembershipMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      id,
      request,
    }: {
      id: string;
      request: UpdateMembershipRequest;
    }) => call('PATCH', `/m3/memberships/${id}`, request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_MEMBERSHIPS] });
    },
  });
}
