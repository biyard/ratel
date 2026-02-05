import { useState } from 'react';
import { useRewardsPageController } from './use-rewards-page-controller';
import { useAdminRewardsI18n, AdminRewardsI18n } from './rewards-page-i18n';
import { useAllTransactions } from './hooks/use-all-transactions';
import {
  RewardCondition,
  RewardAction,
  RewardPeriod,
} from '@/features/spaces/rewards/types';
import {
  ConditionType,
  getConditionType,
  getConditionValue,
} from '@/features/spaces/rewards/types';
import {
  RewardResponse,
  UpdateRewardRequest,
} from './hooks/use-update-reward-mutation';

type TabType = 'rules' | 'transactions';

function getActionLabel(action: RewardAction, i18n: AdminRewardsI18n): string {
  switch (action) {
    case RewardAction.PollRespond:
      return i18n.actionPollRespond;
    default:
      return i18n.actionNone;
  }
}

function getPeriodLabel(period: RewardPeriod, i18n: AdminRewardsI18n): string {
  switch (period) {
    case RewardPeriod.Once:
      return i18n.periodOnce;
    case RewardPeriod.Hourly:
      return i18n.periodHourly;
    case RewardPeriod.Daily:
      return i18n.periodDaily;
    case RewardPeriod.Weekly:
      return i18n.periodWeekly;
    case RewardPeriod.Monthly:
      return i18n.periodMonthly;
    case RewardPeriod.Yearly:
      return i18n.periodYearly;
    case RewardPeriod.Unlimited:
      return i18n.periodUnlimited;
    default:
      return period;
  }
}

function getConditionLabel(
  condition: RewardCondition,
  i18n: AdminRewardsI18n,
): string {
  const conditionType = getConditionType(condition);
  const conditionValue = getConditionValue(condition);
  if (conditionType === ConditionType.None) {
    return i18n.conditionNone;
  }
  if (conditionType === ConditionType.MaxClaims) {
    return `${i18n.conditionMaxClaims}: ${conditionValue}`;
  }
  if (conditionType === ConditionType.MaxPoints) {
    return `${i18n.conditionMaxPoints}: ${conditionValue}`;
  }
  if (conditionType === ConditionType.MaxUserClaims) {
    return `${i18n.conditionMaxUserClaims}: ${conditionValue}`;
  }
  if (conditionType === ConditionType.MaxUserPoints) {
    return `${i18n.conditionMaxUserPoints}: ${conditionValue}`;
  }
  return String(condition);
}

function RewardTable({
  rewards,
  onEdit,
  i18n,
}: {
  rewards: RewardResponse[];
  onEdit: (reward: RewardResponse) => void;
  i18n: AdminRewardsI18n;
}) {
  if (rewards.length === 0) {
    return (
      <div className="py-8 text-center text-gray-500">{i18n.noRewards}</div>
    );
  }
  return (
    <div className="overflow-x-auto">
      <table className="w-full min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
              {i18n.rewardAction}
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
              {i18n.point}
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
              {i18n.period}
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
              {i18n.condition}
            </th>
            <th className="px-6 py-3 text-right text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
              Actions
            </th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
          {rewards.map((reward) => (
            <tr key={reward.reward_action}>
              <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900 dark:text-white">
                {getActionLabel(reward.reward_action, i18n)}
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-300">
                {reward.point.toLocaleString()}
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-300">
                {getPeriodLabel(reward.period, i18n)}
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-300">
                {getConditionLabel(reward.condition, i18n)}
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
                <button
                  onClick={() => onEdit(reward)}
                  className="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300"
                >
                  {i18n.edit}
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function TransactionTable({ i18n }: { i18n: AdminRewardsI18n }) {
  const {
    data,
    isLoading,
    error,
    hasNextPage,
    fetchNextPage,
    isFetchingNextPage,
  } = useAllTransactions();

  const transactions = data?.pages.flatMap((page) => page.items) || [];

  const formatDate = (timestamp: number) => {
    return new Date(timestamp / 1000).toLocaleString();
  };

  const getTransactionTypeColor = (type: string) => {
    switch (type.toUpperCase()) {
      case 'AWARD':
        return 'text-green-600 dark:text-green-400';
      case 'DEDUCT':
        return 'text-red-600 dark:text-red-400';
      case 'TRANSFER':
        return 'text-blue-600 dark:text-blue-400';
      case 'EXCHANGE':
        return 'text-purple-600 dark:text-purple-400';
      default:
        return 'text-gray-600 dark:text-gray-400';
    }
  };

  if (isLoading) {
    return <div className="py-8 text-center text-gray-500">{i18n.loading}</div>;
  }

  if (error) {
    return (
      <div className="py-8 text-center text-red-500">
        {i18n.loadError}: {(error as Error).message}
      </div>
    );
  }

  if (transactions.length === 0) {
    return (
      <div className="py-8 text-center text-gray-500">
        {i18n.noTransactions}
      </div>
    );
  }

  return (
    <div>
      <div className="overflow-x-auto">
        <table className="w-full min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead className="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.userId}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.month}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.transactionType}
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.amount}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.description}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase dark:text-gray-300">
                {i18n.createdAt}
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
            {transactions.map((tx, idx) => (
              <tr key={`${tx.meta_user_id}-${tx.created_at}-${idx}`}>
                <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900 dark:text-white">
                  {tx.meta_user_id.length > 20
                    ? `${tx.meta_user_id.slice(0, 20)}...`
                    : tx.meta_user_id}
                </td>
                <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-300">
                  {tx.month}
                </td>
                <td
                  className={`whitespace-nowrap px-6 py-4 text-sm font-medium ${getTransactionTypeColor(tx.transaction_type)}`}
                >
                  {tx.transaction_type}
                </td>
                <td className="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500 dark:text-gray-300">
                  {tx.amount.toLocaleString()}
                </td>
                <td className="px-6 py-4 text-sm text-gray-500 dark:text-gray-300 max-w-xs truncate">
                  {tx.description || '-'}
                </td>
                <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-300">
                  {formatDate(tx.created_at)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {hasNextPage && (
        <div className="flex justify-center py-4">
          <button
            onClick={() => fetchNextPage()}
            disabled={isFetchingNextPage}
            className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
          >
            {isFetchingNextPage ? i18n.loadingMore : i18n.loadMore}
          </button>
        </div>
      )}
    </div>
  );
}

function getAvailableActions(
  existingRewards: RewardResponse[],
  editingReward: RewardResponse | null,
): RewardAction[] {
  // Get all possible actions from the enum
  const allActions = Object.values(RewardAction);

  // Get the set of already-used actions
  const usedActions = new Set(existingRewards.map((r) => r.reward_action));

  // Filter out used actions, but keep the current reward's action if editing
  return allActions.filter((action) => {
    // If we're editing, always include the current reward's action
    if (editingReward && action === editingReward.reward_action) {
      return true;
    }
    // Otherwise, only include if not already used
    return !usedActions.has(action);
  });
}

function RewardForm({
  reward,
  availableActions,
  onCreate,
  onUpdate,
  onCancel,
  isSubmitting,
  i18n,
}: {
  reward: RewardResponse | null;
  availableActions: RewardAction[];
  onCreate: (request: UpdateRewardRequest) => Promise<void>;
  onUpdate: (request: UpdateRewardRequest) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
  i18n: AdminRewardsI18n;
}) {
  const [action, setAction] = useState<RewardAction>(
    reward?.reward_action || availableActions[0] || RewardAction.PollRespond,
  );
  const [point, setPoint] = useState(reward?.point || 0);
  const [period, setPeriod] = useState<RewardPeriod>(
    reward?.period || RewardPeriod.Once,
  );

  const defaultConditionType = reward
    ? getConditionType(reward.condition)
    : ConditionType.None;
  const defaultConditionValue = reward
    ? getConditionValue(reward.condition)
    : 0;

  const [conditionType, setConditionType] =
    useState<ConditionType>(defaultConditionType);
  const [conditionValue, setConditionValue] = useState(defaultConditionValue);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    let condition: RewardCondition;
    if (conditionType === ConditionType.None) {
      condition = 'None';
    } else {
      condition = { [conditionType]: conditionValue } as RewardCondition;
    }

    const request = {
      action,
      point,
      period,
      condition,
    };

    // Call create or update based on whether we're editing
    if (reward) {
      await onUpdate(request);
    } else {
      await onCreate(request);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-bold text-gray-900 dark:text-white">
          {reward ? i18n.editReward : i18n.addReward}
        </h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.actionLabel}
            </label>
            <select
              value={action}
              onChange={(e) => {
                if (reward) return;
                setAction(e.target.value as RewardAction);
              }}
              disabled={!!reward}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white disabled:opacity-20"
            >
              {availableActions.map((actionOption) => (
                <option key={actionOption} value={actionOption}>
                  {getActionLabel(actionOption, i18n)}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.point}
            </label>
            <input
              type="number"
              value={point}
              onChange={(e) => setPoint(Number(e.target.value))}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.period}
            </label>
            <select
              value={period}
              onChange={(e) => setPeriod(e.target.value as RewardPeriod)}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            >
              <option value={RewardPeriod.Once}>{i18n.periodOnce}</option>
              <option value={RewardPeriod.Hourly}>{i18n.periodHourly}</option>
              <option value={RewardPeriod.Daily}>{i18n.periodDaily}</option>
              <option value={RewardPeriod.Weekly}>{i18n.periodWeekly}</option>
              <option value={RewardPeriod.Monthly}>{i18n.periodMonthly}</option>
              <option value={RewardPeriod.Yearly}>{i18n.periodYearly}</option>
              <option value={RewardPeriod.Unlimited}>
                {i18n.periodUnlimited}
              </option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.condition}
            </label>
            <select
              value={conditionType}
              onChange={(e) =>
                setConditionType(e.target.value as ConditionType)
              }
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            >
              <option value="None">{i18n.conditionNone}</option>
              <option value="MaxClaims">{i18n.conditionMaxClaims}</option>
              <option value="MaxPoints">{i18n.conditionMaxPoints}</option>
              <option value="MaxUserClaims">
                {i18n.conditionMaxUserClaims}
              </option>
              <option value="MaxUserPoints">
                {i18n.conditionMaxUserPoints}
              </option>
            </select>
          </div>

          {conditionType !== 'None' && (
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Value
              </label>
              <input
                type="number"
                value={conditionValue}
                onChange={(e) => setConditionValue(Number(e.target.value))}
                className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              />
            </div>
          )}

          <div className="flex justify-end gap-3 pt-4">
            <button
              type="button"
              onClick={onCancel}
              className="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              {i18n.cancel}
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
            >
              {isSubmitting ? '...' : i18n.save}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export function RewardsPage() {
  const ctrl = useRewardsPageController();
  const i18n = useAdminRewardsI18n();
  const [activeTab, setActiveTab] = useState<TabType>('rules');

  return (
    <div className="mx-auto w-full max-w-desktop p-6">
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-3xl font-bold">{i18n.title}</h1>
      </div>

      {/* Tab Navigation */}
      <div className="mb-4 border-b border-gray-200 dark:border-gray-700">
        <nav className="-mb-px flex gap-4">
          <button
            onClick={() => setActiveTab('rules')}
            className={`py-2 px-4 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'rules'
                ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }`}
          >
            {i18n.tabRules}
          </button>
          <button
            onClick={() => setActiveTab('transactions')}
            className={`py-2 px-4 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'transactions'
                ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }`}
          >
            {i18n.tabTransactions}
          </button>
        </nav>
      </div>

      {/* Tab Content */}
      <div className="rounded-lg bg-white shadow dark:bg-gray-800">
        {activeTab === 'transactions' && <TransactionTable i18n={i18n} />}
        {activeTab === 'rules' && (
          <>
            {ctrl.isLoading ? (
              <div className="py-8 text-center">{i18n.loading}</div>
            ) : ctrl.error ? (
              <div className="py-8 text-center text-red-500">
                {i18n.loadError}: {ctrl.error.message}
              </div>
            ) : (
              <div>
                <div className="flex justify-start px-6 py-6">
                  <button
                    onClick={() => ctrl.openForm()}
                    disabled={
                      getAvailableActions(ctrl.rewards, null).length === 0
                    }
                    className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
                  >
                    {i18n.addReward}
                  </button>
                </div>
                <RewardTable
                  rewards={ctrl.rewards}
                  onEdit={(reward) => ctrl.openForm(reward)}
                  i18n={i18n}
                />
              </div>
            )}
          </>
        )}
      </div>

      {ctrl.isFormOpen && (
        <RewardForm
          reward={ctrl.editingReward}
          availableActions={getAvailableActions(
            ctrl.rewards,
            ctrl.editingReward,
          )}
          onCreate={ctrl.handleCreateReward}
          onUpdate={ctrl.handleUpdateReward}
          onCancel={ctrl.closeForm}
          isSubmitting={ctrl.isSubmitting}
          i18n={i18n}
        />
      )}
    </div>
  );
}
