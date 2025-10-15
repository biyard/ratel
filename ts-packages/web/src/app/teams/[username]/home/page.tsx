'use client';

import { useParams } from 'react-router';
import TeamHome from './page.client';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  if (!username) {
    return <div className="text-center">Team not found</div>;
  }

  return <TeamHome />;
}
