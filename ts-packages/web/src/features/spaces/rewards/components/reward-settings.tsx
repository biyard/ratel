import { useSpaceRewardsController } from '@/app/spaces/[id]/rewards/use-space-rewards-controller';
import { Col } from '@/components/ui/col';
import { FeatureRewardsSection } from './feature-rewards-section';
import { RewardModal } from '../modals/reward-modal';

interface RewardSettingsProps {
  spacePk: string;
}

export function RewardSettings({ spacePk }: RewardSettingsProps) {
  const ctrl = useSpaceRewardsController(spacePk);
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

  const polls = ctrl.polls?.polls ?? [];

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
        <h2 className="text-xl font-bold text-c-wg-100">{t.title}</h2>

        <div className="text-sm text-c-wg-60 mb-2">{t.poll_reward_section}</div>

        <Col className="gap-3">
          {polls.map((poll) => {
            const reward = ctrl.getRewardForPoll(poll.sk);
            return (
              <FeatureRewardsSection
                key={poll.sk}
                i18n={ctrl.i18n}
                poll={poll}
                reward={reward}
                onCreateReward={ctrl.openCreateModal}
                onEditReward={ctrl.openEditModal}
                onDeleteReward={ctrl.handleDelete}
              />
            );
          })}
        </Col>
      </Col>

      <RewardModal
        i18n={ctrl.i18n}
        isOpen={ctrl.isModalOpen.get()}
        onClose={ctrl.closeModal}
        editingReward={ctrl.editingReward.get()}
        onSubmit={ctrl.handleSubmit}
        isSubmitting={
          ctrl.createReward.isPending || ctrl.updateReward.isPending
        }
      />
    </>
  );
}
