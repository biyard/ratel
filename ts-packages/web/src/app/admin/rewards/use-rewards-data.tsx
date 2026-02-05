import {
  useListGlobalRewards,
  useCreateGlobalRewardMutation,
  useUpdateGlobalRewardMutation,
} from '@/features/spaces/rewards/hooks';

export function useRewardsData() {
  const { data, isLoading, error, refetch } = useListGlobalRewards();

  const createMutation = useCreateGlobalRewardMutation();
  const updateMutation = useUpdateGlobalRewardMutation();

  return {
    rewards: data?.items || [],
    isLoading,
    error,
    refetch,
    createReward: createMutation.mutateAsync,
    updateReward: updateMutation.mutateAsync,
    isCreating: createMutation.isPending,
    isUpdating: updateMutation.isPending,
  };
}
