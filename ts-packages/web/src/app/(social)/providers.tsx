import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import {
  getFeedById,
  getNetwork,
  getPosts,
  getPromotion,
  getUserInfo,
} from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { apolloServerClient } from '@/lib/apollo';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const promotion = await getPromotion();

  const data: InitDataOptions[] = [
    await getNetwork(),
    promotion,
    await getUserInfo(),
    await getPosts(1, 10),
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

  const apolloClient = apolloServerClient();
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
