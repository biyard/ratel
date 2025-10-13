'use client';

import { useParams } from 'react-router';
import TeamGroups from './page.client';

export default function Page() {
  const { username } = useParams<{ username: string }>();
  
  if (!username) {
    return <div>Invalid team username</div>;
  }
  
  return <TeamGroups username={username} />;
}
