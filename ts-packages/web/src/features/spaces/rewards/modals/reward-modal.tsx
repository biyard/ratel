import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { RewardForm } from '../components/reward-form';
import { RewardFormData } from '../pages/editor';
import { SpaceRewardsI18n } from '../i18n';
import { RewardConfigItem } from '../hooks/use-reward-config';

interface RewardModalProps {
  i18n: SpaceRewardsI18n;
  configs: RewardConfigItem[] | null;
  isOpen: boolean;
  onClose: () => void;
  editingReward: SpaceRewardResponse | null;
  onSubmit: (data: RewardFormData) => Promise<void>;
  isSubmitting: boolean;
}

export function RewardModal({
  i18n,
  configs,
  isOpen,
  onClose,
  editingReward,
  onSubmit,
  isSubmitting,
}: RewardModalProps) {
  const t = i18n.settings;
  const rewardActions = editingReward
    ? [editingReward.reward_action]
    : (configs?.map((config) => config.reward_action) ?? []);
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
          rewardActions={rewardActions}
          onSubmit={onSubmit}
          onCancel={onClose}
          isSubmitting={isSubmitting}
        />
      </DialogContent>
    </Dialog>
  );
}
