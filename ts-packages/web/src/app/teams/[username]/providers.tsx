import { ReactNode } from 'react';
import { initData } from '@/providers/getQueryClient';
import { getTeamByUsername, getUserInfo } from '@/lib/api/ratel_api.server';
import { getServerQueryClient } from '@/lib/query-utils.server';
import { client } from '@/lib/apollo';
import { ratelApi } from '@/lib/api/ratel_api';
import ClientProviders from './providers.client';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';

export default async function Provider({
  children,
  username,
}: {
  children: ReactNode;
  username: string;
}) {
  const queryClient = await getServerQueryClient();

  const {
    data: { users },
  } = await client.query(ratelApi.graphql.getTeamByTeamname(username));

  if (users.length === 0) {
    return <></>;
  }

  const userId = users[0].id;

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
