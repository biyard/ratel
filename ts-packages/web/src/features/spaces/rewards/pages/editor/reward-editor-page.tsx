import { SpacePathProps } from '@/features/space-path-props';
import { useRewardEditorController } from './reward-editor-controller';
import { Col } from '@/components/ui/col';
import { RewardModal } from '../../modals/reward-modal';
import { RewardSection } from '../../components/reward-section';

export function RewardEditorPage({ spacePk }: SpacePathProps) {
  const ctrl = useRewardEditorController(spacePk);
  const t = ctrl.i18n.settings;

  if (ctrl.rewardFeatures.length === 0) {
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
        </div>

        <Col className="gap-4">
          {ctrl.rewardFeatures.map((rewardFeature) => (
            <RewardSection
              key={`${rewardFeature.action}-${rewardFeature.entityKey}`}
              spacePk={spacePk}
              title={rewardFeature.title}
              action={rewardFeature.action}
              entityKey={rewardFeature.entityKey}
              onAddSpaceReward={ctrl.openCreateModal}
              onEditSpaceReward={ctrl.openEditModal}
              onDeleteSpaceReward={ctrl.handleDelete}
            />
          ))}
        </Col>
      </Col>

      <RewardModal
        i18n={ctrl.i18n}
        rewards={ctrl.targetRewards.get()}
        isOpen={ctrl.isModalOpen.get()}
        onClose={ctrl.closeModal}
        editingReward={ctrl.editingReward.get()}
        onSubmit={ctrl.handleSubmit}
        isSubmitting={
          ctrl.createSpaceReward.isPending || ctrl.updateSpaceReward.isPending
        }
      />
    </>
  );
}
