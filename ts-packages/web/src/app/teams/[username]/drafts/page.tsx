import { ratelApi } from '@/lib/api/ratel_api';
import TeamDraftPage from './page.client';
import { client } from '@/lib/apollo';
import { FeedStatus } from '@/lib/api/models/feeds';
import { prefetchInfiniteFeeds } from '@/hooks/feeds/use-feeds-infinite-query';
export interface TeamLayoutProps {
  params: Promise<{ username: string }>;
}

export default async function Page({ params }: TeamLayoutProps) {
  const { username } = await params;
  const {
    data: { users },
  } = await client.query(ratelApi.graphql.getTeamByTeamname(username));

  if (users.length === 0) {
    // FIXME: fix this to use not-found.tsx
    return <div className="text-center">Team not found</div>;
  }

  await Promise.allSettled([
    prefetchInfiniteFeeds(users[0].id, FeedStatus.Draft),
  ]);

  return <TeamDraftPage teamId={users[0].id} username={username} />;
}
