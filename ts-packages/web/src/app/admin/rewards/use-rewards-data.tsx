import {
  useListGlobalRewards,
  useUpdateGlobalRewardMutation,
} from '@/features/spaces/rewards/hooks';

export function useRewardsData() {
  const { data, isLoading, error, refetch } = useListGlobalRewards();

  const updateMutation = useUpdateGlobalRewardMutation();

  return {
    rewards: data?.items || [],
    isLoading,
    error,
    refetch,
    updateReward: updateMutation.mutateAsync,
    isUpdating: updateMutation.isPending,
  };
}
