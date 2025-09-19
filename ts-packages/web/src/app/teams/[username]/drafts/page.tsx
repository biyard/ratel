import { ratelApi } from '@/lib/api/ratel_api';
import TeamDraftPage from './page.client';
import { FeedStatus } from '@/lib/api/models/feeds';
import { prefetchInfiniteFeeds } from '@/hooks/feeds/use-feeds-infinite-query';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import NotfoundPage from '@/app/not-found';

export interface TeamLayoutProps {
  params: Promise<{ username: string }>;
}

export default async function Page({ params }: TeamLayoutProps) {
  const { username } = await params;
  const userResp = await apiFetch<{ id: number } | null>(
    `${config.api_url}${ratelApi.users.getUserByUsername(username)}`,
    { ignoreError: true, cache: 'no-store' },
  );

  if (!userResp?.data?.id) {
    // FIXME: fix this to use not-found.tsx
    return <NotfoundPage />;
  }

  await Promise.allSettled([
    prefetchInfiniteFeeds(userResp.data.id, FeedStatus.Draft),
  ]);

  return <TeamDraftPage teamId={userResp.data.id} username={username} />;
}
