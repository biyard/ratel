import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { initData } from '@/providers/getQueryClient';
import { getSpaceById } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';

export default async function Provider({
  children,
  spaceId,
}: {
  children: ReactNode;
  spaceId: number;
}) {
  const queryClient = await getServerQueryClient();

  try {
    // Initialize the query client with the space data
    initData(queryClient, [await getSpaceById(spaceId)]);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  return <ClientProviders spaceId={spaceId}>{children}</ClientProviders>;
}
