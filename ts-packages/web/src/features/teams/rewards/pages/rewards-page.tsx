import { Suspense } from 'react';
import { useTeamRewardsI18n } from '../i18n';
import { useTeamRewardsPageController } from './use-rewards-page-controller';
import { PointsSummaryCard } from '@/features/rewards/components/points-summary-card';
import { ExchangePreviewCard } from '@/features/rewards/components/exchange-preview-card';
import { TransactionList } from '@/features/rewards/components/transaction-list';

interface RewardsPageProps {
  username: string;
}

function RewardsPageContent({ username }: RewardsPageProps) {
  const ctrl = useTeamRewardsPageController(username);
  const i18n = useTeamRewardsI18n();

  // Check if user is a team member
  if (!ctrl.permissions) {
    return (
      <div className="w-full max-w-desktop mx-auto px-4 py-8">
        <div className="bg-card-bg border border-card-border rounded-lg p-8">
          <div className="text-center text-destructive">
            You must be a team member to view rewards
          </div>
        </div>
      </div>
    );
  }

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
            {i18n.error}: {ctrl.rewardsError?.message}
          </div>
        </div>
      </div>
    );
  }

  const rewards = ctrl.rewards;
  const estimatedTokens =
    rewards.total_points > 0
      ? Math.round(
          (rewards.team_points / rewards.total_points) *
            rewards.monthly_token_supply,
        )
      : 0;

  return (
    <div
      data-testid="team-rewards-page"
      className="w-full max-w-desktop mx-auto px-4 py-6"
    >
      <PointsSummaryCard
        i18n={i18n}
        totalPoints={rewards.total_points}
        userPoints={rewards.team_points}
        monthlyTokenSupply={rewards.monthly_token_supply}
        estimatedTokens={estimatedTokens}
        tokenSymbol={rewards.token_symbol}
      />
      <ExchangePreviewCard
        i18n={i18n}
        totalPoints={rewards.team_points}
        estimatedTokens={estimatedTokens}
        name={ctrl.team.nickname}
        tokenSymbol={rewards.token_symbol}
      />

      <div className="mt-6">
        <TransactionList
          i18n={i18n}
          transactions={ctrl.transactions}
          isLoading={ctrl.isLoadingTransactions}
          hasNextPage={ctrl.hasNextPage}
          isFetchingNextPage={ctrl.isFetchingNextPage}
          fetchNextPage={ctrl.fetchNextPage}
        />
      </div>
    </div>
  );
}

export default function RewardsPage({ username }: RewardsPageProps) {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <RewardsPageContent username={username} />
    </Suspense>
  );
}
