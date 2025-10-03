import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import {
  getFeedById,
  getNetwork,
  getPromotion,
  getUserInfo,
  listNews,
} from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { QK_INF_POSTS } from '@/constants';
import { listPosts } from '@/lib/api/ratel/posts.v3';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const network = await getNetwork();
  const promotion = await getPromotion();
  const user = await getUserInfo();

  const news = await listNews();

  const data: InitDataOptions[] = [network, promotion, user, news];

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

  const posts = await listPosts();
  queryClient.setQueryData([QK_INF_POSTS], {
    pages: [posts],
    pageParams: [posts.bookmark],
  });
  /* await Promise.allSettled([prefetchInfiniteFeeds()]); */

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
