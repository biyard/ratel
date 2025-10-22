import { QK_MEMBERSHIPS } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { DeleteMembershipResponse } from '../dto/delete-membership-response';

/**
 * Delete a membership (Admin only)
 */
export function useDeleteMembershipMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string): Promise<DeleteMembershipResponse> =>
      call('DELETE', `/m3/memberships/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_MEMBERSHIPS] });
    },
  });
}
