import MembersPage from '@/features/teams/members/pages/members-page';
import { useParams } from 'react-router';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    return <div>Username not found</div>;
  }
  return <MembersPage username={username!} />;
}
