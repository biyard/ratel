import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { Edit2Icon, Trash2Icon } from 'lucide-react';
import { RewardsI18n } from '../i18n';

interface RewardCardProps {
  i18n: RewardsI18n;
  reward: SpaceRewardResponse;
  onEdit: () => void;
  onDelete: () => void;
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

export function RewardCard({
  i18n,
  reward,
  onEdit,
  onDelete,
}: RewardCardProps) {
  const t = i18n.settings;

  return (
    <div className="border border-c-wg-20 rounded-lg p-4 bg-c-bg-card">
      <Col className="gap-3">
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <h4 className="text-lg font-semibold text-c-wg-100">
              {getRewardLabel(reward, t)}
            </h4>
            {reward.description && (
              <p className="text-sm text-c-wg-60 mt-1">{reward.description}</p>
            )}
          </div>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={onEdit}
              aria-label={t.edit_reward}
            >
              <Edit2Icon className="w-4 h-4" />
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={onDelete}
              aria-label={t.delete_reward}
            >
              <Trash2Icon className="w-4 h-4" />
            </Button>
          </div>
        </div>

        <div className="flex gap-6 text-sm">
          <div>
            <span className="text-c-wg-60">{t.credits}: </span>
            <span className="font-medium text-c-wg-100">{reward.credits}</span>
          </div>
          <div>
            <span className="text-c-wg-60">{t.total_claims}: </span>
            <span className="font-medium text-c-wg-100">
              {reward.total_claims}
            </span>
          </div>
          <div>
            <span className="text-c-wg-60">{t.total_points}: </span>
            <span className="font-medium text-c-wg-100">
              {reward.total_points}
            </span>
          </div>
        </div>
      </Col>
    </div>
  );
}
