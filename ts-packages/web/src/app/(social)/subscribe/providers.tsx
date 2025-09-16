import { getUserInfo } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { ReactNode } from 'react';
import ClientProviders from './provider.client';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const user = await getUserInfo();

  const data: InitDataOptions[] = [user];

  try {
    initData(queryClient, data);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  const dehydratedState = dehydrate(queryClient);
  return (
    <HydrationBoundary state={dehydratedState}>
      <ClientProviders>{children}</ClientProviders>
    </HydrationBoundary>
  );
}
