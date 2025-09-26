import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { initData } from '@/providers/getQueryClient';
import {
  getPermission,
  getRedeemCode,
  getSpaceById,
  getTeamByUsername,
} from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { prefetchFeedById } from '@/hooks/feeds/use-feed-by-id';
import { GroupPermission } from '@/lib/api/models/group';

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

  const team = await getTeamByUsername(space.data?.author[0].username ?? '');

  const writePostPermission = await getPermission(
    team.data?.id ?? 0,
    GroupPermission.WritePosts,
  );

  const deletePostPermission = await getPermission(
    team.data?.id ?? 0,
    GroupPermission.DeletePosts,
  );

  try {
    // Initialize the query client with the space data
    initData(queryClient, [
      space,
      redeemCode,
      team,
      writePostPermission,
      deletePostPermission,
    ]);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  await Promise.allSettled([prefetchFeedById(feedId)]);

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders spaceId={spaceId}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
