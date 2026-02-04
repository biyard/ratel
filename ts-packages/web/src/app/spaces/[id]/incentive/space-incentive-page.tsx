import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceIncentivePage } from '@/features/spaces/dao/pages/incentive/space-incentive-page';

export default function SpaceIncentiveRoutePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!spacePk) {
    return null;
  }

  if (!space) {
    throw new Error('Space not found');
  }

  return <SpaceIncentivePage spacePk={spacePk} />;
}
