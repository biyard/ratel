import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoViewerController } from './space-dao-viewer-controller';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { useRefreshSpaceDaoTokensMutation } from '@/features/spaces/dao/hooks/use-refresh-space-dao-tokens-mutation';
import { useEffect, useMemo, useState } from 'react';
import { config } from '@/config';

export function SpaceDaoViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoViewerPage: spacePk=${spacePk}`);
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const { data: tokenList, isLoading: tokensLoading } = useSpaceDaoTokens(
    spacePk,
    50,
    Boolean(dao?.contract_address),
  );
  const refreshTokens = useRefreshSpaceDaoTokensMutation(spacePk);
  const [selectedToken, setSelectedToken] = useState<string | null>(null);
  const [didRefreshTokens, setDidRefreshTokens] = useState(false);

  const orderedTokens = useMemo(() => {
    const items = tokenList?.items ?? [];
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt || items.length === 0) {
      return items;
    }
    return [...items].sort((a, b) => {
      const aIsUsdt = a.token_address.toLowerCase() === usdt;
      const bIsUsdt = b.token_address.toLowerCase() === usdt;
      if (aIsUsdt === bIsUsdt) return 0;
      return aIsUsdt ? -1 : 1;
    });
  }, [tokenList?.items]);

  const selectedTokenItem =
    orderedTokens.find(
      (item) =>
        item.token_address.toLowerCase() === selectedToken?.toLowerCase(),
    ) ?? null;
  const fallbackIsUsdt =
    Boolean(selectedToken && config.usdt_address) &&
    selectedToken?.toLowerCase() === config.usdt_address?.toLowerCase();
  const selectedTokenBalance =
    selectedTokenItem?.balance ?? (fallbackIsUsdt ? '0' : null);
  const selectedTokenDecimals =
    selectedTokenItem?.decimals ?? (fallbackIsUsdt ? 6 : null);

  const ctrl = useSpaceDaoViewerController(
    spacePk,
    dao,
    selectedToken,
    selectedTokenBalance,
    selectedTokenDecimals,
  );

  useEffect(() => {
    if (selectedToken) return;
    const items = orderedTokens;
    if (items.length) {
      const usdt = config.usdt_address?.toLowerCase();
      const usdtItem = usdt
        ? items.find((item) => item.token_address.toLowerCase() === usdt)
        : null;
      setSelectedToken(
        (usdtItem?.token_address ?? items[0].token_address) || null,
      );
      return;
    }
    if (config.usdt_address) {
      setSelectedToken(config.usdt_address);
    }
  }, [selectedToken, orderedTokens]);

  useEffect(() => {
    if (!dao?.contract_address || didRefreshTokens) return;
    refreshTokens.mutate();
    setDidRefreshTokens(true);
  }, [dao?.contract_address, didRefreshTokens, refreshTokens]);

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
        recipientCount={ctrl.chainRecipientCount.get()}
        rewardRecipients={ctrl.visibleRewardRecipients}
        rewardRemainingCount={ctrl.rewardMeta?.remaining_count ?? null}
        rewardTotalCount={ctrl.rewardMeta?.total_count ?? null}
        rewardLoading={ctrl.rewardRecipientsLoading}
        showRewardRecipients={Boolean(ctrl.space?.isFinished)}
        showEdit={false}
        currentUserEvm={ctrl.currentUserEvm}
        claimableAmount={ctrl.perRecipientDisplay}
        isClaimable={ctrl.canClaimReward}
        isClaiming={ctrl.isClaiming.get()}
        onClaimReward={async (rewardSk) => {
          await ctrl.handleClaimReward(rewardSk);
          refreshTokens.mutate();
        }}
        tokens={orderedTokens}
        selectedToken={selectedToken}
        onSelectToken={setSelectedToken}
        tokensLoading={tokensLoading}
        onRefreshTokens={() => refreshTokens.mutate()}
        isRefreshingTokens={refreshTokens.isPending}
      />
    </div>
  );
}
