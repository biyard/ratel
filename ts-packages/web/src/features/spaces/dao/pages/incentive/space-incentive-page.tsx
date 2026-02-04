import { SpacePathProps } from '@/features/space-path-props';
import { useTranslation } from 'react-i18next';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import Card from '@/components/card';
import { useEffect, useMemo, useState } from 'react';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { config } from '@/config';
import { useSpaceIncentiveController } from './space-incentive-controller';
import { Button } from '@/components/ui/button';

const isZeroBalance = (balance?: string | null) =>
  !balance || /^0+$/.test(balance);

export function SpaceIncentivePage({ spacePk }: SpacePathProps) {
  const { t } = useTranslation('SpaceDaoEditor');
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const { data: tokenList } = useSpaceDaoTokens(
    spacePk,
    50,
    Boolean(dao?.contract_address),
  );
  const [selectedToken, setSelectedToken] = useState<string | null>(null);

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

  const selectedTokenItem =
    orderedTokens.find(
      (item) =>
        item.token_address.toLowerCase() === selectedToken?.toLowerCase(),
    ) ?? null;
  const fallbackIsUsdt =
    Boolean(selectedToken && config.usdt_address) &&
    selectedToken?.toLowerCase() === config.usdt_address?.toLowerCase();
  const selectedTokenDecimals =
    selectedTokenItem?.decimals ?? (fallbackIsUsdt ? 6 : null);

  useEffect(() => {
    if (selectedToken || orderedTokens.length === 0) return;
    const usdt = config.usdt_address?.toLowerCase();
    const usdtItem = usdt
      ? orderedTokens.find((item) => item.token_address.toLowerCase() === usdt)
      : null;
    setSelectedToken(
      (usdtItem?.token_address ?? orderedTokens[0].token_address) || null,
    );
  }, [orderedTokens, selectedToken]);

  const ctrl = useSpaceIncentiveController(
    spacePk,
    dao,
    selectedToken,
    selectedTokenDecimals,
  );
  const currentUserItem = ctrl.currentUserItem;

  if (isLoading || !dao) {
    return null;
  }

  return (
    <div className="flex flex-col w-full max-w-[1152px] gap-5">
      <Card>
        <div className="space-y-4 w-full">
          <div>
            <h1 className="text-2xl font-semibold text-text-primary">
              {t('dao_incentive_title')}
            </h1>
          </div>

          {currentUserItem ? (
            <div className="rounded-xl border border-input-box-border bg-background px-4 py-4">
              <div className="grid grid-cols-1 gap-3 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-text-secondary">
                    {t('dao_incentive_wallet')}
                  </span>
                  <span className="text-text-primary">
                    {ctrl.currentUserEvm ?? '-'}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-text-secondary">
                    {t('dao_incentive_amount')}
                  </span>
                  <span className="text-text-primary">
                    {ctrl.perRecipientDisplay ?? '-'}
                  </span>
                </div>
                <div className="flex items-center justify-end pt-2">
                  {currentUserItem.claimed ? (
                    <span className="text-text-secondary text-xs">
                      {t('dao_selected_claimed')}
                    </span>
                  ) : (
                    <Button
                      type="button"
                      variant="rounded_primary"
                      size="sm"
                      onClick={ctrl.handleClaim}
                      disabled={ctrl.isClaiming.get()}
                    >
                      {ctrl.isClaiming.get()
                        ? t('dao_selected_claiming')
                        : t('dao_selected_claim')}
                    </Button>
                  )}
                </div>
              </div>
            </div>
          ) : null}

          <div className="rounded-xl border border-input-box-border bg-background px-4 py-4">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-text-secondary">
                {t('dao_selected_title')}
              </p>
            </div>
            {ctrl.recipientsLoading ? (
              <div className="text-sm text-text-secondary">
                {t('dao_selected_loading')}
              </div>
            ) : ctrl.recipients.length === 0 ? (
              <div className="text-sm text-text-secondary">
                {t('dao_selected_empty')}
              </div>
            ) : (
              <table className="w-full table-fixed text-sm">
                <thead className="text-text-secondary">
                  <tr className="border-b border-border">
                    <th className="px-2 py-2 text-left font-medium">
                      {t('dao_selected_evm')}
                    </th>
                    <th className="px-2 py-2 text-right font-medium">
                      {t('dao_selected_amount')}
                    </th>
                    <th className="px-2 py-2 text-right font-medium">
                      {t('dao_selected_status')}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {ctrl.recipients.map((item) => (
                    <tr
                      key={item.address}
                      className="border-b border-border last:border-0"
                    >
                      <td className="px-2 py-2 text-text-primary">
                        {item.address}
                      </td>
                      <td className="px-2 py-2 text-right text-text-primary">
                        {ctrl.perRecipientDisplay ?? '-'}
                      </td>
                      <td className="px-2 py-2 text-right text-text-secondary">
                        {item.claimed
                          ? t('dao_selected_claimed')
                          : t('dao_selected_pending')}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </div>
      </Card>
    </div>
  );
}
