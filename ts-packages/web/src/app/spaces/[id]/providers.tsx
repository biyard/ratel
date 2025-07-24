import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { initData } from '@/providers/getQueryClient';
import { getFeedById, getSpaceById } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate } from '@tanstack/react-query';

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

  const feed = await getFeedById(feedId);

  try {
    // Initialize the query client with the space data
    initData(queryClient, [space, feed]);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders spaceId={spaceId} dehydratedState={dehydratedState}>
      {children}
    </ClientProviders>
  );
}
