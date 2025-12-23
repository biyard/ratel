import { useSpaceRewardsController } from '@/app/spaces/[id]/rewards/use-space-rewards-controller';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { RewardCard } from './reward-card';
import { RewardModal } from '../modals/reward-modal';
import useAvailableRewards from '../hooks/use-available-rewards';
import usePoll from '@/features/spaces/polls/hooks/use-poll';
import { PlusIcon } from 'lucide-react';

interface RewardEditorProps {
  spacePk: string;
}

export function RewardEditor({ spacePk }: RewardEditorProps) {
  const ctrl = useSpaceRewardsController(spacePk);
  const { data: availableRewards } = useAvailableRewards('poll');
  const { data: pollsData } = usePoll(spacePk);
  const t = ctrl.i18n.settings;

  if (!ctrl.isPollSpace) {
    return (
      <div className="flex items-center justify-center p-8">
        <Col crossAxisAlignment="center" className="gap-2">
          <p className="text-c-wg-60">{t.no_polls}</p>
          <p className="text-sm text-c-wg-40">{t.no_polls_description}</p>
        </Col>
      </div>
    );
  }

  const polls = pollsData?.polls ?? [];
  const rewards = ctrl.rewards.items;

  if (polls.length === 0) {
    return (
      <div className="flex items-center justify-center p-8">
        <Col crossAxisAlignment="center" className="gap-2">
          <p className="text-c-wg-60">{t.no_polls}</p>
          <p className="text-sm text-c-wg-40">{t.no_polls_description}</p>
        </Col>
      </div>
    );
  }

  return (
    <>
      <Col className="gap-4 p-4">
        <div className="flex justify-between items-center">
          <h2 className="text-xl font-bold text-c-wg-100">{t.title}</h2>
          <Button variant="outline" size="sm" onClick={ctrl.openCreateModal}>
            <PlusIcon className="w-4 h-4" />
            {t.create_reward}
          </Button>
        </div>

        <div className="text-sm text-c-wg-60 mb-2">{t.poll_reward_section}</div>

        {rewards.length === 0 ? (
          <div className="text-center py-8 text-c-wg-60 text-sm border border-dashed border-c-wg-20 rounded-lg">
            {t.no_rewards}
            <br />
            <span className="text-xs">{t.no_rewards_description}</span>
          </div>
        ) : (
          <Col className="gap-3">
            {rewards.map((reward) => (
              <RewardCard
                key={reward.sk}
                i18n={ctrl.i18n}
                reward={reward}
                onEdit={() => ctrl.openEditModal(reward)}
                onDelete={() => ctrl.handleDelete(reward)}
              />
            ))}
          </Col>
        )}
      </Col>

      <RewardModal
        i18n={ctrl.i18n}
        isOpen={ctrl.isModalOpen.get()}
        onClose={ctrl.closeModal}
        editingReward={ctrl.editingReward.get()}
        availableRewards={availableRewards.items}
        polls={polls}
        onSubmit={ctrl.handleSubmit}
        isSubmitting={
          ctrl.createReward.isPending || ctrl.updateReward.isPending
        }
      />
    </>
  );
}
