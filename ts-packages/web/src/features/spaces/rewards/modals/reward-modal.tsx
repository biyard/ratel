import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { SpaceRewardResponse } from '../types';
import { RewardForm } from '../components/reward-form';
import { RewardFormData } from '../pages/editor';
import { SpaceRewardsI18n } from '../i18n';
import { Reward } from '../hooks/use-rewards';

interface RewardModalProps {
  i18n: SpaceRewardsI18n;
  rewards: Reward[] | null;
  isOpen: boolean;
  onClose: () => void;
  editingReward: SpaceRewardResponse | null;
  onSubmit: (data: RewardFormData) => Promise<void>;
  isSubmitting: boolean;
}

export function RewardModal({
  i18n,
  rewards,
  isOpen,
  onClose,
  editingReward,
  onSubmit,
  isSubmitting,
}: RewardModalProps) {
  const t = i18n.settings;
  const rewardBehaviors = editingReward
    ? [editingReward.behavior]
    : (rewards?.map((reward) => reward.reward_behavior) ?? []);
  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {editingReward ? t.edit_reward : t.create_reward}
          </DialogTitle>
        </DialogHeader>

        <RewardForm
          i18n={i18n}
          initialData={editingReward}
          rewardBehaviors={rewardBehaviors}
          onSubmit={onSubmit}
          onCancel={onClose}
          isSubmitting={isSubmitting}
        />
      </DialogContent>
    </Dialog>
  );
}
