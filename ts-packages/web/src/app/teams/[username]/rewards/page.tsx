import RewardsPage from '@/features/teams/rewards/pages/rewards-page';
import { useParams } from 'react-router';

export default function TeamRewardsPage() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    throw new Error('Username is required');
  }

  return <RewardsPage username={username} />;
}
