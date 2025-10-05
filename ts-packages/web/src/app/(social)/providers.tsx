import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import { getNetwork, getUserInfo, listNews } from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { prefetchInfinitePosts } from './_hooks/use-infinite-posts';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const network = await getNetwork();
  const user = await getUserInfo();

  const news = await listNews();

  const data: InitDataOptions[] = [network, user, news];

  try {
    // Initialize the query client with the space data
    initData(queryClient, data);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  await Promise.allSettled([prefetchInfinitePosts()]);

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
