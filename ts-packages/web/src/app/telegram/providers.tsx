import { ReactNode } from 'react';
import { initData, InitDataOptions } from '@/providers/getQueryClient';
import { getUserInfo } from '@/lib/api/ratel_api.server';
import ClientProviders from './providers.client';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { dehydrate } from '@tanstack/react-query';

export default async function Provider({ children }: { children: ReactNode }) {
  const queryClient = await getServerQueryClient();
  const user = await getUserInfo();

  const data: InitDataOptions[] = [user];

  try {
    // Initialize the query client with the space data
    initData(queryClient, data);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders dehydratedState={dehydratedState}>
      {children}
    </ClientProviders>
  );
}
