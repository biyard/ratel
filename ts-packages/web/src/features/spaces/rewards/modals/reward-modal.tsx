import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { RewardForm } from '../components/reward-form';
import { RewardFormData } from '@/app/spaces/[id]/rewards/use-space-rewards-controller';
import { RewardsI18n } from '../i18n';

interface RewardModalProps {
  i18n: RewardsI18n;
  isOpen: boolean;
  onClose: () => void;
  editingReward: SpaceRewardResponse | null;
  onSubmit: (data: RewardFormData) => Promise<void>;
  isSubmitting: boolean;
}

export function RewardModal({
  i18n,
  isOpen,
  onClose,
  editingReward,
  onSubmit,
  isSubmitting,
}: RewardModalProps) {
  const t = i18n.settings;

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
          onSubmit={onSubmit}
          onCancel={onClose}
          isSubmitting={isSubmitting}
        />
      </DialogContent>
    </Dialog>
  );
}
