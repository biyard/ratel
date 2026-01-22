import { RewardsI18n } from '../types';
import { PointTransactionResponse } from '@/app/rewards/types';
import { TransactionItem } from './transaction-item';

interface TransactionListProps {
  i18n: RewardsI18n;
  transactions: PointTransactionResponse[] | undefined;
  isLoading: boolean;
  hasNextPage: boolean;
  isFetchingNextPage: boolean;
  fetchNextPage: () => void;
  formatPoints: (points: number) => string;
}

export function TransactionList({
  i18n,
  transactions,
  isLoading,
  hasNextPage,
  isFetchingNextPage,
  fetchNextPage,
  formatPoints,
}: TransactionListProps) {
  if (isLoading) {
    return (
      <div className="py-8 text-center text-text-primary">{i18n.loading}</div>
    );
  }

  if (!transactions || transactions.length === 0) {
    return (
      <div className="py-16 text-center">
        <h3 className="text-lg font-semibold text-white mb-2">{i18n.empty}</h3>
        <p className="text-sm text-text-primary">{i18n.empty_description}</p>
      </div>
    );
  }

  return (
    <div data-testid="transaction-list" className="flex flex-col gap-0">
      {transactions.map((transaction, index) => (
        <TransactionItem
          key={`${transaction.created_at}-${index}`}
          i18n={i18n}
          transaction={transaction}
          formatPoints={formatPoints}
        />
      ))}

      {hasNextPage && (
        <button
          onClick={fetchNextPage}
          disabled={isFetchingNextPage}
          className="mt-4 py-3 text-center text-sm font-medium text-text-primary hover:text-white transition-colors disabled:opacity-50"
        >
          {isFetchingNextPage ? i18n.loading : i18n.load_more}
        </button>
      )}
    </div>
  );
}
