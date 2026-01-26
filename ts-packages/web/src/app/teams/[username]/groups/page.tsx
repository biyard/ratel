import TeamGroupsPage from '@/features/teams/groups/pages/page';
import { useParams } from 'react-router';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    return <div>Username not found</div>;
  }
  return <TeamGroupsPage username={username!} />;
}
