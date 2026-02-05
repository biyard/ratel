import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { SpaceRewardResponse } from '../types';
import { Edit2Icon, Trash2Icon } from 'lucide-react';
import { SpaceRewardsI18n } from '../i18n';
import { getRewardUserBehaviorI18nKey } from '../types/reward-user-behavior';

interface RewardCardProps {
  i18n: SpaceRewardsI18n;
  reward: SpaceRewardResponse;
  onEdit: () => void;
  onDelete: () => void;
}

export function RewardCard({
  i18n,
  reward,
  onEdit,
  onDelete,
}: RewardCardProps) {
  const t = i18n.settings;
  console.log('reward.behavior', reward.behavior);
  return (
    <div
      data-testid="reward-card"
      className="border border-c-wg-20 rounded-lg p-4 bg-c-bg-card"
    >
      <Col className="gap-3">
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <h4 className="text-lg font-semibold text-c-wg-100">
              {t[
                getRewardUserBehaviorI18nKey(reward.behavior) as keyof typeof t
              ] || reward.behavior}
            </h4>
            {reward.description && (
              <p className="text-sm text-c-wg-60 mt-1">{reward.description}</p>
            )}
          </div>
          <div className="flex gap-2">
            <Button
              data-testid="reward-edit-button"
              variant="outline"
              size="sm"
              onClick={onEdit}
              aria-label={t.edit_reward}
            >
              <Edit2Icon className="w-4 h-4" />
            </Button>
            <Button
              data-testid="reward-delete-button"
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
