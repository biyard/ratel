import { Col } from '@/components/ui/col';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { ListRewardsResponse } from '../types/list-rewards-response';
import { RewardsI18n, useRewardsI18n } from '../i18n';
import { GiftIcon } from 'lucide-react';

interface RewardViewerProps {
  rewards: ListRewardsResponse;
}

function getRewardLabel(
  reward: SpaceRewardResponse,
  t: RewardsI18n['settings'],
): string {
  const rewardType = reward.getRewardType();
  switch (rewardType) {
    case 'poll_respond':
      return t.poll_respond_reward;
    case 'board_comment':
      return t.board_comment_reward;
    case 'board_like':
      return t.board_like_reward;
    default:
      return t.unknown_reward;
  }
}

function RewardViewCard({
  reward,
  t,
}: {
  reward: SpaceRewardResponse;
  t: RewardsI18n['settings'];
}) {
  return (
    <div className="border border-c-wg-20 rounded-lg p-4 bg-c-bg-card">
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 rounded-full bg-c-primary/10 flex items-center justify-center">
          <GiftIcon className="w-5 h-5 text-c-primary" />
        </div>
        <div className="flex-1">
          <h4 className="text-base font-semibold text-c-wg-100">
            {getRewardLabel(reward, t)}
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

export function RewardViewer({ rewards }: RewardViewerProps) {
  const i18n = useRewardsI18n();
  const t = i18n.settings;

  if (rewards.items.length === 0) {
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
      <h2 className="text-xl font-bold text-c-wg-100">{i18n.sidemenu.title}</h2>

      <Col className="gap-3">
        {rewards.items.map((reward) => (
          <RewardViewCard key={reward.sk} reward={reward} t={t} />
        ))}
      </Col>
    </Col>
  );
}
