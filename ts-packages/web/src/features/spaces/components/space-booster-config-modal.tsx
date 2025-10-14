import SpaceConfigForm from '@/features/spaces/components/space-config-form';
import { SpaceTypeSelectFormController } from '../hooks/use-space-type-select-form-controller';

export type SpaceBoosterConfigModalProps =
  React.HTMLAttributes<HTMLDivElement> & {
    ctrl: SpaceTypeSelectFormController;
  };

export default function SpaceBoosterConfigModal({
  ctrl,
}: SpaceBoosterConfigModalProps) {
  return (
    <div className="w-full max-w-[95vw] max-tablet:w-fit">
      <div className="max-mobile:w-full">
        <SpaceConfigForm
          spaceType={ctrl.selectedSpace.type}
          onBack={ctrl.handleBackToSelection}
          onConfirm={(startedAt, endedAt, boosterType) => {
            return ctrl.handleCreateSpace({
              spaceType: ctrl.selectedSpace.type,
              postPk: ctrl.feed_id,
              startedAt,
              endedAt,
              boosterType,
            });
          }}
          isLoading={ctrl.isLoading.get()}
        />
      </div>
    </div>
  );
}
