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

  return {
    ...data,
    formatPoints,
    formatTokens,
  };
}
