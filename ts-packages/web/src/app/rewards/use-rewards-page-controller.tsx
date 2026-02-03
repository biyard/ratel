import { useRewardsData } from './use-rewards-data';

export function useRewardsPageController() {
  const data = useRewardsData();

  return {
    ...data,
  };
}
