import { useMyRewards } from './_hooks/use-my-rewards';
import { useMyPointTransactions } from './_hooks/use-my-point-transactions';
import { MyRewardsResponse, PointTransactionResponse } from './types';

export interface RewardsData {
  rewards: MyRewardsResponse | undefined;
  transactions: PointTransactionResponse[] | undefined;
  isLoadingRewards: boolean;
  isLoadingTransactions: boolean;
  rewardsError: Error | null;
  transactionsError: Error | null;
  hasNextPage: boolean;
  fetchNextPage: () => void;
  isFetchingNextPage: boolean;
}

export function useRewardsData(month?: string): RewardsData {
  const {
    data: rewards,
    isLoading: isLoadingRewards,
    error: rewardsError,
  } = useMyRewards(month);

  const {
    data: transactionsData,
    isLoading: isLoadingTransactions,
    error: transactionsError,
    hasNextPage,
    fetchNextPage,
    isFetchingNextPage,
  } = useMyPointTransactions(month);

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
