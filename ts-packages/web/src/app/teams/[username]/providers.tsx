import { ReactNode } from 'react';
import { initData } from '@/providers/getQueryClient';
import { getTeamByUsername, getUserInfo } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { ratelApi } from '@/lib/api/ratel_api';
import ClientProviders from './providers.client';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';

export default async function Provider({
  children,
  username,
}: {
  children: ReactNode;
  username: string;
}) {
  const queryClient = await getServerQueryClient();

  // Lookup user by username using REST v2
  const userResp = await apiFetch<{ id: number } | null>(
    `${config.api_url}${ratelApi.users.getUserByUsername(username)}`,
    { ignoreError: true, cache: 'no-store' },
  );

  if (!userResp?.data?.id) {
    return <></>;
  }
  const userId = userResp.data.id;

  const team = await getTeamByUsername(username);
  const user = await getUserInfo();

  try {
    // Initialize the query client with the space data
    initData(queryClient, [team, user]);
  } catch (error) {
    console.error('Failed to fetch data', error);
    throw error;
  }

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders userId={userId}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
