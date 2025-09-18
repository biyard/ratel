import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import {
  getFeedById,
  getNetwork,
  getPromotion,
  getUserInfo,
} from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { FeedStatus } from '@/lib/api/models/feeds';
import { prefetchInfiniteFeeds } from '@/hooks/feeds/use-feeds-infinite-query';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const network = await getNetwork();
  const promotion = await getPromotion();
  const user = await getUserInfo();

  const data: InitDataOptions[] = [network, promotion, user];

  if (promotion.data) {
    data.push(await getFeedById(promotion.data.feed_id));
  }

  try {
    // Initialize the query client with the space data
    initData(queryClient, data);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  await Promise.allSettled([prefetchInfiniteFeeds(0, FeedStatus.Published)]);

  const dehydratedState = dehydrate(queryClient);
  
  // TODO: Remove Apollo cache after full migration to REST v2
  // @deprecated - Migrating to REST v2
  const apolloCache = '{}';
  try {
    // Prefetch news using REST v2
    await apiFetch(`${config.api_url}${ratelApi.news.list(3)}`, {
      ignoreError: true,
      cache: 'no-store',
    });
  } catch (error) {
    console.error('Failed to fetch news', error);
  }

  return (
    <ClientProviders apolloCache={apolloCache}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
