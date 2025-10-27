import { useListMemberships } from '@/features/membership/hooks/use-list-memberships';
import { useCreateMembershipMutation } from '@/features/membership/hooks/use-create-membership-mutation';
import { useUpdateMembershipMutation } from '@/features/membership/hooks/use-update-membership-mutation';
import { useDeleteMembershipMutation } from '@/features/membership/hooks/use-delete-membership-mutation';
import { usePurchaseMembershipMutation } from '@/features/membership/hooks/use-purchase-membership-mutation';

export function useMembershipsData() {
  const { data, isLoading, error, refetch } = useListMemberships();

  const createMutation = useCreateMembershipMutation();

  const updateMutation = useUpdateMembershipMutation();

  const deleteMutation = useDeleteMembershipMutation();

  const purchaseMutation = usePurchaseMembershipMutation();

  return {
    memberships: data?.items || [],
    total: data?.items.length || 0,
    isLoading,
    error,
    refetch,
    createMembership: createMutation.mutateAsync,
    updateMembership: updateMutation.mutateAsync,
    deleteMembership: deleteMutation.mutateAsync,
    purchaseMembership: purchaseMutation.mutateAsync,
    isCreating: createMutation.isPending,
    isUpdating: updateMutation.isPending,
    isDeleting: deleteMutation.isPending,
  };
}
