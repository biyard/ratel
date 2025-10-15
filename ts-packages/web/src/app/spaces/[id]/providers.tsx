import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { initData } from '@/providers/getQueryClient';
import { getRedeemCode, getSpaceById } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { prefetchFeedById } from '@/hooks/feeds/use-feed-by-id';

export default async function Provider({
  children,
  spaceId,
}: {
  children: ReactNode;
  spaceId: number;
}) {
  const queryClient = await getServerQueryClient();

  const space = await getSpaceById(spaceId);
  const feedId = space.data?.feed_id ?? 0;

  const redeemCode = await getRedeemCode(spaceId);

  // TODO: Migrate spaces to v3 API
  // - Use getTeamDetailByUsernameV3 to fetch team
  // - Use embedded permissions from team detail instead of separate permission checks

  try {
    // Initialize the query client with the space data
    initData(queryClient, [space, redeemCode]);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  // TODO: Update space API to use string feed_id in v3
  await Promise.allSettled([prefetchFeedById(feedId.toString())]);

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders spaceId={spaceId}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
