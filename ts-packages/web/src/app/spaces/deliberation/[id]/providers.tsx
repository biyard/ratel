import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { getServerQueryClient } from '@/lib/query-utils.server';

export default async function Provider({
  children,
  spaceId,
}: {
  children: ReactNode;
  spaceId: string;
}) {
  const queryClient = await getServerQueryClient();
  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders spaceId={spaceId}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
