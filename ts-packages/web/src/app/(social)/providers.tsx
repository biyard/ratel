import { ReactNode } from 'react';
import { initData } from '@/providers/getQueryClient';
import {
  getFeedById,
  getNetwork,
  getPosts,
  getPromotion,
  getUserInfo,
} from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const promotion = await getPromotion();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const data: any[] = [
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

  return <ClientProviders>{children}</ClientProviders>;
}
