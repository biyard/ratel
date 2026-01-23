import { ArrowsArrowRight } from '@/components/icons';
import { RewardsI18n } from '../types';
import Card from '@/components/card';
import { formatPoints, formatTokens } from './utils';

interface ExchangePreviewCardProps {
  i18n: RewardsI18n;
  name: string;
  totalPoints: number;
  estimatedTokens: number;
  tokenSymbol: string;
  canExchange?: boolean; // Optional: Admin-only token exchange capability
  exchangeButtonText?: string; // Optional: Custom button text
  onExchangeClick?: () => void; // Optional: Exchange button click handler
}

export function ExchangePreviewCard({
  i18n,
  name,
  totalPoints,
  estimatedTokens,
  tokenSymbol,

  canExchange = true,
  exchangeButtonText,
  onExchangeClick,
}: ExchangePreviewCardProps) {
  return (
    <div className="border-t border-bg pt-10">
      <Card className="flex flex-col items-center gap-5 p-4">
        <div className="flex items-center justify-between gap-4 w-full">
          <div className="flex flex-col gap-0.5">
            <div className="flex items-center gap-1">
              <div className="w-5 h-5 rounded-full bg-primary" />
              <span className="text-[15px] font-medium text-white">
                {formatPoints(totalPoints)} P
              </span>
            </div>
            <div className="flex items-start gap-1 flex-col">
              <span className="text-sm font-semibold text-text-primary">
                {i18n.exchange_from}
              </span>
              <span className="text-sm font-semibold text-white">
                {name} {i18n.point}
              </span>
            </div>
          </div>

          <div className="flex items-center justify-center">
            <ArrowsArrowRight className="size-6 [&>path]:stroke-primary" />
          </div>

          <div className="flex flex-col items-end gap-0.5">
            <div className="flex items-center gap-1">
              <span className="text-[15px] font-medium text-text-primary">
                {formatTokens(estimatedTokens)} {tokenSymbol}
              </span>
              <div className="w-5 h-5 rounded-full bg-primary" />
            </div>
            <div className="flex items-end gap-2 flex-col">
              <span className="text-sm font-semibold text-text-primary">
                {i18n.exchange_to}
              </span>
              <span className="text-sm font-semibold text-white">
                {name} {i18n.token}
              </span>
            </div>
          </div>
        </div>

        {/* Info Message or Exchange Button */}
        {onExchangeClick && canExchange ? (
          <button
            onClick={onExchangeClick}
            className="w-full py-2 px-4 bg-primary hover:bg-primary/90 text-white rounded-lg font-medium transition-colors"
          >
            {exchangeButtonText || 'Exchange'}
          </button>
        ) : (
          <p className="text-xs font-medium text-text-primary text-center">
            {i18n.swap_available_message}
          </p>
        )}
      </Card>
    </div>
  );
}
