import { ArrowUp, ArrowsExchange } from '@/components/icons';
import { RewardsI18n } from '../types';

interface PointsSummaryCardProps {
  i18n: RewardsI18n;
  tokenSymbol: string;
  totalPoints: number;
  userPoints: number;
  monthlyTokenSupply: number;
  estimatedTokens: number;
  formatPoints: (points: number) => string;
  formatTokens: (tokens: number) => string;
}

export function PointsSummaryCard({
  i18n,
  totalPoints,
  userPoints,
  tokenSymbol,
  monthlyTokenSupply,
  estimatedTokens,
  formatPoints,
  formatTokens,
}: PointsSummaryCardProps) {
  const sharePercentage =
    totalPoints > 0 ? ((userPoints / totalPoints) * 100).toFixed(2) : '0';

  return (
    <div
      data-testid="points-summary-card"
      className="bg-[#1A1A1A] rounded-xl p-5"
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-5">
        <h2 className="text-base font-semibold text-white tracking-wide">
          {i18n.title}
        </h2>
        <ArrowUp className="w-5 h-5 text-white" />
      </div>

      <div className="flex justify-between items-start mb-5">
        <div className="flex flex-col gap-2">
          <span className="text-sm font-semibold text-text-primary">
            {i18n.your_share}
          </span>
          <span
            data-testid="user-points-value"
            className="text-xl font-bold text-white"
          >
            {formatPoints(userPoints)} P
          </span>
          <div className="flex items-center gap-1">
            <ArrowsExchange className="w-5 h-5 [&>path]:stroke-white" />
            <span className="text-[15px] font-medium text-white">
              {formatTokens(estimatedTokens)} {tokenSymbol} ({sharePercentage}%)
            </span>
          </div>
        </div>

        <div className="flex flex-col items-end gap-2">
          <span className="text-sm font-semibold text-text-primary">
            {i18n.this_months_pool}
          </span>
          <span className="text-xl font-bold text-text-primary">
            {formatTokens(monthlyTokenSupply)} {tokenSymbol} (100%)
          </span>
        </div>
      </div>

      <div className="relative h-9 bg-[#262626] rounded-lg overflow-hidden">
        <div
          className="absolute left-0 top-0 h-full bg-primary rounded-lg transition-all duration-300"
          style={{ width: `${Math.max(parseFloat(sharePercentage), 0.5)}%` }}
        />
        <div className="absolute left-3 top-1/2 -translate-y-1/2 text-sm font-bold text-white">
          Yours {sharePercentage}%
        </div>
        <div className="absolute right-3 top-1/2 -translate-y-1/2 text-sm font-bold text-white">
          100%
        </div>
      </div>
    </div>
  );
}
