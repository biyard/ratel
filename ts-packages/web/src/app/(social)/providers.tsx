import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import {
  getFeedById,
  getNetwork,
  getPostByUserId,
  getPromotion,
  getUserInfo,
  prefetchPostInfinite,
} from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { client } from '@/lib/apollo';
import { FeedStatus } from '@/lib/api/models/feeds';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const network = await getNetwork();
  const promotion = await getPromotion();
  const user = await getUserInfo();
  const post = await prefetchPostInfinite(10);

  const myPosts = await getPostByUserId(user.data?.id ?? 0, 1, 20);
  const myDraftPosts = await getPostByUserId(
    user.data?.id ?? 0,
    1,
    20,
    FeedStatus.Draft,
  );

  const data: InitDataOptions[] = [
    network,
    promotion,
    user,
    post,
    myPosts,
    myDraftPosts,
  ];

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

  const dehydratedState = dehydrate(queryClient);

  const apolloClient = client;
  const newsQuery = ratelApi.graphql.listNews(3);
  await apolloClient.query({
    query: newsQuery.query,
    variables: newsQuery.variables,
  });

  const apolloCache = JSON.stringify(apolloClient.extract());

  return (
    <ClientProviders
      dehydratedState={dehydratedState}
      apolloCache={apolloCache}
    >
      {children}
    </ClientProviders>
  );
}
