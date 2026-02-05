import { SpacePathProps } from '@/features/space-path-props';
import { useRewardViewerController } from './reward-viewer-controller';
import { Col } from '@/components/ui/col';
import { SpaceRewardResponse } from '../../types';
import { SpaceRewardsI18n } from '../../i18n';
import { GiftIcon } from 'lucide-react';
import { getRewardUserBehaviorI18nKey } from '../../types/reward-user-behavior';

function RewardViewCard({
  reward,
  t,
}: {
  reward: SpaceRewardResponse;
  t: SpaceRewardsI18n['settings'];
}) {
  return (
    <div className="border border-c-wg-20 rounded-lg p-4 bg-c-bg-card">
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 rounded-full bg-c-primary/10 flex items-center justify-center">
          <GiftIcon className="w-5 h-5 text-c-primary" />
        </div>
        <div className="flex-1">
          <h4 className="text-base font-semibold text-c-wg-100">
            {t[
              getRewardUserBehaviorI18nKey(reward.behavior) as keyof typeof t
            ] || reward.behavior}{' '}
          </h4>
          {reward.description && (
            <p className="text-sm text-c-wg-60">{reward.description}</p>
          )}
        </div>
        <div className="text-right">
          <div className="text-lg font-bold text-c-primary">
            {reward.points.toLocaleString()}
          </div>
          <div className="text-xs text-c-wg-60">{t.total_points}</div>
        </div>
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
