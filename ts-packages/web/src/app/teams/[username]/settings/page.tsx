import { useParams } from 'react-router';
import TeamSettingsPage from '@/features/teams/settings/pages/team-settings-page';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    return <div>Invalid team username</div>;
  }

  return <TeamSettingsPage username={username} />;
}
