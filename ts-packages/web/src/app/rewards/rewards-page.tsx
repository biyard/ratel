import { useMyRewardsI18n } from './rewards-page-i18n';
import { useRewardsPageController } from './use-rewards-page-controller';
import { PointsSummaryCard } from './components/points-summary-card';
import { ExchangePreviewCard } from './components/exchange-preview-card';
import { TransactionList } from './components/transaction-list';

export default function RewardsPage() {
  const ctrl = useRewardsPageController();
  const i18n = useMyRewardsI18n();
  if (ctrl.isLoadingRewards) {
    return (
      <div className="w-full max-w-desktop mx-auto px-4 py-8">
        <div className="text-center text-foreground">{i18n.loading}</div>
      </div>
    );
  }

  if (ctrl.rewardsError || !ctrl.rewards) {
    return (
      <div className="w-full max-w-desktop mx-auto px-4 py-8">
        <div className="bg-card-bg border border-card-border rounded-lg p-8">
          <div className="text-center text-destructive">
            {i18n.error}: {ctrl.rewardsError.message}
          </div>
        </div>
      </div>
    );
  }

  const rewards = ctrl.rewards;
  console.log('rewards', rewards);
  const estimatedTokens = Math.round(
    (rewards.user_points / rewards.total_points) * rewards.monthly_token_supply,
  );
  return (
    <div
      data-testid="my-rewards-page"
      className="w-full max-w-desktop mx-auto px-4 py-6"
    >
      <PointsSummaryCard
        i18n={i18n}
        totalPoints={rewards.total_points}
        userPoints={rewards.user_points}
        monthlyTokenSupply={rewards.monthly_token_supply}
        estimatedTokens={estimatedTokens}
        tokenSymbol={rewards.token_symbol}
        formatPoints={ctrl.formatPoints}
        formatTokens={ctrl.formatTokens}
      />
      <ExchangePreviewCard
        i18n={i18n}
        totalPoints={rewards.user_points}
        estimatedTokens={estimatedTokens}
        name={rewards.project_name}
        tokenSymbol={rewards.token_symbol}
        formatPoints={ctrl.formatPoints}
        formatTokens={ctrl.formatTokens}
      />
      <div className="mt-6">
        <TransactionList
          i18n={i18n}
          transactions={ctrl.transactions}
          isLoading={ctrl.isLoadingTransactions}
          hasNextPage={ctrl.hasNextPage}
          isFetchingNextPage={ctrl.isFetchingNextPage}
          fetchNextPage={ctrl.fetchNextPage}
          formatPoints={ctrl.formatPoints}
        />
      </div>
    </div>
  );
}
