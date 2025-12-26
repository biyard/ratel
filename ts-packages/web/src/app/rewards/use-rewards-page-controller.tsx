import { useRewardsData } from './use-rewards-data';

export function useRewardsPageController() {
  const data = useRewardsData();

  const formatPoints = (points: number): string => {
    return new Intl.NumberFormat().format(points);
  };

  const formatTokens = (tokens: number): string => {
    return new Intl.NumberFormat(undefined, {
      maximumFractionDigits: 2,
    }).format(tokens);
  };

  const calculateSharePercentage = (): string => {
    if (!data.rewards) return '0';
    const { total_points, exchange_ratio } = data.rewards;
    if (exchange_ratio === 0) return '0';
    // exchange_ratio = monthly_token_supply / project_total_points
    // user's share = total_points / project_total_points * 100
    // = total_points * exchange_ratio / monthly_token_supply * 100
    const percentage =
      (total_points * exchange_ratio * 100) /
      (data.rewards.estimated_tokens || 1);
    return percentage.toFixed(2);
  };

  return {
    ...data,
    formatPoints,
    formatTokens,
    calculateSharePercentage,
  };
}
