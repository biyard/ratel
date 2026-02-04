import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceDaoViewerController } from './space-dao-viewer-controller';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { useRefreshSpaceDaoTokensMutation } from '@/features/spaces/dao/hooks/use-refresh-space-dao-tokens-mutation';
import { useEffect, useMemo, useState } from 'react';
import { config } from '@/config';

const isZeroBalance = (balance?: string | null) =>
  !balance || /^0+$/.test(balance);

export function SpaceDaoViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoViewerPage: spacePk=${spacePk}`);
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const {
    data: tokenList,
    isLoading: tokensLoading,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useSpaceDaoTokens(spacePk, 5, Boolean(dao?.contract_address));
  const refreshTokens = useRefreshSpaceDaoTokensMutation(spacePk);
  const [selectedToken, setSelectedToken] = useState<string | null>(null);
  const [didRefreshTokens, setDidRefreshTokens] = useState(false);
  const [tokenPageIndex, setTokenPageIndex] = useState(0);

  const allTokens = useMemo(
    () => tokenList?.pages.flatMap((page) => page.items) ?? [],
    [tokenList?.pages],
  );
  const filteredTokens = useMemo(() => {
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt) return allTokens;
    return allTokens.filter((item) => {
      const isUsdt = item.token_address.toLowerCase() === usdt;
      if (!isUsdt) return true;
      return !isZeroBalance(item.balance);
    });
  }, [allTokens]);

  const orderedTokens = useMemo(() => {
    const items = filteredTokens;
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
  }, [filteredTokens]);
  const hasAnyTokens = filteredTokens.length > 0;

  const tokenPages = tokenList?.pages ?? [];
  const tokenPage = tokenPages[tokenPageIndex] ?? tokenPages[0];
  const visibleTokens = useMemo(() => {
    const items =
      tokenPage?.items?.filter((item) => {
        const usdt = config.usdt_address?.toLowerCase();
        if (!usdt) return true;
        const isUsdt = item.token_address.toLowerCase() === usdt;
        if (!isUsdt) return true;
        return !isZeroBalance(item.balance);
      }) ?? [];
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt || items.length === 0) return items;
    return [...items].sort((a, b) => {
      const aIsUsdt = a.token_address.toLowerCase() === usdt;
      const bIsUsdt = b.token_address.toLowerCase() === usdt;
      if (aIsUsdt === bIsUsdt) return 0;
      return aIsUsdt ? -1 : 1;
    });
  }, [tokenPage, tokenPage?.items]);
  const hasPrevPage = tokenPageIndex > 0;
  const canGoNext =
    tokenPageIndex < tokenPages.length - 1 || Boolean(hasNextPage);

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

  useEffect(() => {
    if (tokenPageIndex > 0 && tokenPageIndex > tokenPages.length - 1) {
      setTokenPageIndex(Math.max(tokenPages.length - 1, 0));
    }
  }, [tokenPageIndex, tokenPages.length]);

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
        incentiveRecipients={ctrl.visibleIncentiveRecipients}
        incentiveRemainingCount={ctrl.incentiveMeta?.remaining_count ?? null}
        incentiveTotalCount={ctrl.incentiveMeta?.total_count ?? null}
        incentiveLoading={ctrl.incentiveRecipientsLoading}
        showIncentiveRecipients={Boolean(ctrl.space?.isFinished)}
        showEdit={false}
        currentUserEvm={ctrl.currentUserEvm}
        claimableAmount={ctrl.perRecipientDisplay}
        isClaimable={ctrl.canClaimIncentive}
        isClaiming={ctrl.isClaiming.get()}
        onClaimIncentive={async (incentiveSk) => {
          await ctrl.handleClaimIncentive(incentiveSk);
          refreshTokens.mutate();
        }}
        tokens={visibleTokens}
        tokenHasAny={hasAnyTokens}
        tokensLoading={tokensLoading}
        onRefreshTokens={() => refreshTokens.mutate()}
        isRefreshingTokens={refreshTokens.isPending}
        tokenHasPrev={hasPrevPage}
        tokenHasNext={canGoNext}
        isFetchingNextTokenPage={isFetchingNextPage}
        onPrevTokens={() => setTokenPageIndex((prev) => Math.max(prev - 1, 0))}
        onNextTokens={async () => {
          if (tokenPageIndex < tokenPages.length - 1) {
            setTokenPageIndex((prev) => prev + 1);
            return;
          }
          if (hasNextPage) {
            await fetchNextPage();
            setTokenPageIndex((prev) => prev + 1);
          }
        }}
      />
    </div>
  );
}
