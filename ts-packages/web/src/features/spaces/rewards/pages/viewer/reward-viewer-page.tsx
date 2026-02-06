import { SpacePathProps } from '@/features/space-path-props';
import { useRewardViewerController } from './reward-viewer-controller';
import { Col } from '@/components/ui/col';
import { SpaceRewardResponse } from '../../types';
import { SpaceRewardsI18n } from '../../i18n';
import { GiftIcon, Infinity as InfinityIcon } from 'lucide-react';
import {
  useRewardBehaviorLabel,
  useRewardPeriodLabel,
} from '../../types/reward-i18n';
import { Badge } from '@/components/ui/badge';
import { useTranslation } from 'react-i18next';
import { RewardPeriod } from '../../types/reward-period';
import {
  ConditionType,
  getConditionType,
  getConditionValue,
} from '../../types/reward-condition';
import { cn } from '@/lib/utils';
import RoundCheckIcon from '@/components/round-checkicon';

type ClaimStatus =
  | { type: 'claimed' }
  | {
      type: 'available';
      current: number;
      max: number;
      unit: 'claims' | 'points';
    }
  | { type: 'unlimited' };

function getClaimStatus(reward: SpaceRewardResponse): ClaimStatus {
  const condType = getConditionType(reward.condition);
  const condValue = getConditionValue(reward.condition);

  if (reward.period === RewardPeriod.Once) {
    return reward.user_claims >= 1
      ? { type: 'claimed' }
      : {
          type: 'available',
          current: reward.user_claims,
          max: 1,
          unit: 'claims',
        };
  }

  if (condType === ConditionType.MaxUserClaims && condValue !== null) {
    return reward.user_claims >= condValue
      ? { type: 'claimed' }
      : {
          type: 'available',
          current: reward.user_claims,
          max: condValue,
          unit: 'claims',
        };
  }

  if (condType === ConditionType.MaxUserPoints && condValue !== null) {
    return reward.user_points >= condValue
      ? { type: 'claimed' }
      : {
          type: 'available',
          current: reward.user_points,
          max: condValue,
          unit: 'points',
        };
  }

  return { type: 'unlimited' };
}

function RewardViewCard({
  reward,
  t: _t,
}: {
  reward: SpaceRewardResponse;
  t: SpaceRewardsI18n['settings'];
}) {
  const getBehaviorLabel = useRewardBehaviorLabel();
  const getPeriodLabel = useRewardPeriodLabel();
  const { t: tReward } = useTranslation('RewardTypes');
  const status = getClaimStatus(reward);

  const isClaimed = status.type === 'claimed';
  const perClaimPoints = reward.points * (reward.credits || 1);

  return (
    <div
      className={cn(
        'border-2 rounded-lg p-4 bg-c-bg-card',
        isClaimed ? 'border-primary/40' : 'border-c-wg-20',
      )}
    >
      <div className="flex items-center gap-3">
        <div
          className={cn(
            'w-10 h-10 rounded-full flex items-center justify-center shrink-0',
            isClaimed ? 'bg-transparent' : 'bg-c-primary/10',
          )}
        >
          {isClaimed ? (
            <RoundCheckIcon />
          ) : (
            <GiftIcon className="size-8 text-c-primary" />
          )}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <h4 className="text-base font-semibold text-c-wg-100 truncate">
              {getBehaviorLabel(reward.behavior)}
            </h4>
            {isClaimed ? (
              <Badge
                variant="outline"
                size="sm"
                className="border-amber-500/60 bg-amber-500/10 text-amber-600 dark:text-amber-400"
              >
                {tReward('reward_claimed')}
              </Badge>
            ) : (
              <Badge
                variant="outline"
                size="sm"
                className="border-c-primary/60 bg-c-primary/10 text-c-primary"
              >
                {getPeriodLabel(reward.period)}
              </Badge>
            )}
          </div>
          {reward.description && (
            <p className="text-sm text-c-wg-60 truncate">
              {reward.description}
            </p>
          )}
        </div>
      </div>

      {status.type === 'available' && (
        <div className="mt-3 flex items-center gap-3">
          <div className="flex-1 h-2 bg-text-secondary/50 rounded-full overflow-hidden">
            <div
              className="h-full bg-text-secondary rounded-full transition-all duration-300"
              style={{
                width: `${Math.min((status.current / status.max) * 100, 100)}%`,
              }}
            />
          </div>
          <span className="text-xs text-c-wg-60 whitespace-nowrap">
            {status.current} / {status.max}{' '}
            {tReward(status.unit === 'claims' ? 'claims_unit' : 'points_unit')}
          </span>
        </div>
      )}

      {status.type === 'claimed' && (
        <div className="mt-3 flex items-center gap-3">
          <div className="flex-1 h-2 rounded-full overflow-hidden">
            <div className="h-full bg-primary rounded-full w-full" />
          </div>
          <span className="text-xs text-c-wg-60 whitespace-nowrap">
            {tReward('completed')}
          </span>
        </div>
      )}

      {status.type === 'unlimited' && (
        <div className="mt-3 flex items-center gap-2">
          <InfinityIcon className="w-4 h-4 text-c-wg-40" />
          <span className="text-xs text-c-wg-60">{tReward('no_limit')}</span>
        </div>
      )}

      <div className="mt-3 flex items-center justify-between text-sm">
        <span className="text-c-wg-60">
          {tReward('per_claim')}:{' '}
          <span className="font-semibold text-c-wg-100">
            {perClaimPoints.toLocaleString()} P
          </span>
        </span>
        <span className="text-c-wg-60">
          {tReward('earned')}:{' '}
          <span className="font-semibold text-c-primary">
            {reward.user_points.toLocaleString()} P
          </span>
        </span>
      </div>
    </div>
  );
}

export function RewardViewerPage({ spacePk }: SpacePathProps) {
  const ctrl = useRewardViewerController(spacePk);
  const t = ctrl.i18n.settings;
  const rewards = ctrl.spaceRewards;

  if (rewards.length === 0) {
    return (
      <div className="flex items-center justify-center p-8">
        <Col crossAxisAlignment="center" className="gap-2">
          <GiftIcon className="w-12 h-12 text-c-wg-40" />
          <p className="text-c-wg-60">{t.no_rewards}</p>
        </Col>
      </div>
    );
  }

  return (
    <Col className="gap-4 p-4">
      <h2 className="text-xl font-bold text-c-wg-100">
        {ctrl.i18n.sidemenu.title}
      </h2>

      <Col className="gap-3">
        {rewards.map((reward) => (
          <RewardViewCard key={reward.sk} reward={reward} t={t} />
        ))}
      </Col>
    </Col>
  );
}
