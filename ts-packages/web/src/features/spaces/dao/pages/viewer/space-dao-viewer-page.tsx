import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoViewerController } from './space-dao-viewer-controller';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { useRefreshSpaceDaoTokensMutation } from '@/features/spaces/dao/hooks/use-refresh-space-dao-tokens-mutation';
import { useEffect, useState } from 'react';
import { config } from '@/config';

export function SpaceDaoViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoViewerPage: spacePk=${spacePk}`);
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const ctrl = useSpaceDaoViewerController(spacePk, dao);
  const { data: tokenList, isLoading: tokensLoading } = useSpaceDaoTokens(
    spacePk,
    50,
    Boolean(dao?.contract_address),
  );
  const refreshTokens = useRefreshSpaceDaoTokensMutation(spacePk);
  const [selectedToken, setSelectedToken] = useState<string | null>(null);

  useEffect(() => {
    if (!selectedToken && tokenList?.items?.length) {
      setSelectedToken(tokenList.items[0].token_address);
      return;
    }
    if (!selectedToken && !tokenList?.items?.length && config.usdt_address) {
      setSelectedToken(config.usdt_address);
    }
  }, [selectedToken, tokenList?.items]);

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
        samplingCount={ctrl.chainSamplingCount.get()}
        samples={ctrl.visibleSamples}
        samplesBookmark={ctrl.samples?.bookmark ?? null}
        samplesLoading={ctrl.samplesLoading}
        canPrevSample={ctrl.canPrevSample}
        canNextSample={ctrl.canNextSample}
        onPrevSample={ctrl.handlePrevSample}
        onNextSample={ctrl.handleNextSample}
        showSamples={Boolean(ctrl.space?.isFinished)}
        showEdit={false}
        canDistributeReward={ctrl.canDistributeReward}
        isDistributingPage={ctrl.isDistributingPage.get()}
        tokens={tokenList?.items ?? []}
        selectedToken={selectedToken}
        onSelectToken={setSelectedToken}
        tokensLoading={tokensLoading}
        onRefreshTokens={() => refreshTokens.mutate()}
        isRefreshingTokens={refreshTokens.isPending}
      />
    </div>
  );
}
