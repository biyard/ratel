import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoViewerController } from './space-dao-viewer-controller';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';

export function SpaceDaoViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoViewerPage: spacePk=${spacePk}`);
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const ctrl = useSpaceDaoViewerController(spacePk, dao);

  if (isLoading) {
    return null;
  }

  if (!dao) {
    return null;
  }

  return (
    <div className="flex flex-col w-full max-w-[1152px] gap-5">
      <SpaceDaoInfoCard
        dao={dao}
        balance={ctrl.balance.get()}
        balanceLoading={ctrl.balanceLoading.get()}
        isDepositOpen={ctrl.isDepositOpen.get()}
        depositAmount={ctrl.depositAmount.get()}
        isDepositing={ctrl.isDepositing.get()}
        onOpenDeposit={ctrl.handleOpenDeposit}
        onCloseDeposit={ctrl.handleCloseDeposit}
        onDepositAmountChange={ctrl.handleDepositAmountChange}
        onConfirmDeposit={ctrl.handleConfirmDeposit}
      />
    </div>
  );
}
