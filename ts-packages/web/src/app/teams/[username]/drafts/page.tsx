'use client';

import { useParams } from 'react-router';
import TeamDraftPage from './page.client';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    return <div className="text-center">Invalid team username</div>;
  }

  return <TeamDraftPage username={username} />;
}
