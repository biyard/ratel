import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import useRewards from '@/features/spaces/rewards/hooks/use-rewards';
import usePoll from '@/features/spaces/polls/hooks/use-poll';
import { SpaceType } from '@/features/spaces/types/space-type';

export function useSpaceRewardsData(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: rewards } = useRewards(spacePk);

  const isPollSpace =
    space?.spaceType === SpaceType.Poll ||
    space?.spaceType === SpaceType.Deliberation;

  const { data: pollsData } = usePoll(spacePk);

  return {
    space,
    rewards,
    polls: isPollSpace ? pollsData : null,
    isPollSpace,
  };
}
