import { useTeamRewards } from '@/features/teams/rewards/hooks/use-team-rewards';
import { useTeamPointTransactions } from '@/features/teams/rewards/hooks/use-team-point-transactions';
import {
  TeamRewardsResponse,
  PointTransactionResponse,
} from '@/features/teams/rewards/types';

export interface TeamRewardsData {
  rewards: TeamRewardsResponse | undefined;
  transactions: PointTransactionResponse[] | undefined;
  isLoadingRewards: boolean;
  isLoadingTransactions: boolean;
  rewardsError: Error | null;
  transactionsError: Error | null;
  hasNextPage: boolean;
  fetchNextPage: () => void;
  isFetchingNextPage: boolean;
}

export function useTeamRewardsData(
  teamPk: string,
  month?: string,
): TeamRewardsData {
  const {
    data: rewards,
    isLoading: isLoadingRewards,
    error: rewardsError,
  } = useTeamRewards(teamPk, month);

  const {
    data: transactionsData,
    isLoading: isLoadingTransactions,
    error: transactionsError,
    hasNextPage,
    fetchNextPage,
    isFetchingNextPage,
  } = useTeamPointTransactions(teamPk, month);

  const transactions = transactionsData?.pages.flatMap((page) => page.items);

  return {
    rewards,
    transactions,
    isLoadingRewards,
    isLoadingTransactions,
    rewardsError: rewardsError as Error | null,
    transactionsError: transactionsError as Error | null,
    hasNextPage: hasNextPage ?? false,
    fetchNextPage,
    isFetchingNextPage,
  };
}
