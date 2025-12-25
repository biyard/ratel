import { useState } from 'react';
import { useRewardsPageController } from './use-rewards-page-controller';
import { useAdminRewardsI18n, AdminRewardsI18n } from './rewards-page-i18n';
import type {
  GlobalRewardResponse,
  GlobalRewardAction,
  UpdateGlobalRewardRequest,
  RewardCondition,
} from '@/features/spaces/rewards/types';
import {
  ConditionType,
  getConditionType,
  getConditionValue,
  RewardPeriod,
} from '@/features/spaces/rewards/types';

function getActionLabel(
  action: GlobalRewardAction,
  i18n: AdminRewardsI18n,
): string {
  switch (action) {
    case 'PollRespond':
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
  rewards: GlobalRewardResponse[];
  onEdit: (reward: GlobalRewardResponse) => void;
  i18n: AdminRewardsI18n;
}) {
  if (rewards.length === 0) {
    return (
      <div className="py-8 text-center text-gray-500">{i18n.noRewards}</div>
    );
  }
  console.log('REAWRD', rewards);
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

function RewardEditForm({
  reward,
  onSubmit,
  onCancel,
  isSubmitting,
  i18n,
}: {
  reward: GlobalRewardResponse;
  onSubmit: (request: UpdateGlobalRewardRequest) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
  i18n: AdminRewardsI18n;
}) {
  const [point, setPoint] = useState(reward.point);
  const [period, setPeriod] = useState<RewardPeriod>(reward.period);

  const defaultConditionType = getConditionType(reward.condition);
  const defaultConditionValue = getConditionValue(reward.condition);

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

    await onSubmit({
      action: reward.reward_action,
      point,
      period,
      condition,
    });
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-bold text-gray-900 dark:text-white">
          {i18n.edit}: {getActionLabel(reward.reward_action, i18n)}
        </h2>
        <form onSubmit={handleSubmit} className="space-y-4">
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

  if (ctrl.isLoading) {
    return (
      <div className="mx-auto w-full max-w-desktop p-6">
        <div className="py-8 text-center">{i18n.loading}</div>
      </div>
    );
  }

  if (ctrl.error) {
    return (
      <div className="mx-auto w-full max-w-desktop p-6">
        <div className="py-8 text-center text-red-500">
          {i18n.loadError}: {ctrl.error.message}
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto w-full max-w-desktop p-6">
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-3xl font-bold">{i18n.title}</h1>
      </div>

      <div className="rounded-lg bg-white shadow dark:bg-gray-800">
        <RewardTable
          rewards={ctrl.rewards}
          onEdit={ctrl.openEditForm}
          i18n={i18n}
        />
      </div>

      {ctrl.isFormOpen && ctrl.editingReward && (
        <RewardEditForm
          reward={ctrl.editingReward}
          onSubmit={ctrl.handleUpdateReward}
          onCancel={ctrl.closeForm}
          isSubmitting={ctrl.isSubmitting}
          i18n={i18n}
        />
      )}
    </div>
  );
}
