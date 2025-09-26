import TeamDraftPage from './page.client';
import { FeedStatus } from '@/lib/api/models/feeds';
import { prefetchInfiniteFeeds } from '@/hooks/feeds/use-feeds-infinite-query';
import { getTeamByUsername } from '@/lib/api/ratel_api.server';
export interface TeamLayoutProps {
  params: Promise<{ username: string }>;
}

export default async function Page({ params }: TeamLayoutProps) {
  const { username } = await params;
  const user = await getTeamByUsername(username);

  if (user == null) {
    // FIXME: fix this to use not-found.tsx
    return <div className="text-center">Team not found</div>;
  }

  await Promise.allSettled([
    prefetchInfiniteFeeds(user?.data?.id ?? 0, FeedStatus.Draft),
  ]);

  return <TeamDraftPage teamId={user?.data?.id ?? 0} username={username} />;
}
