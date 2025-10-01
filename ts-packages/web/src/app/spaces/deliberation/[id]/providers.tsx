import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { getDeliberationSpaceById } from '@/lib/api/ratel_api.server';
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
  const spacePk = 'DELIBERATION_SPACE#' + spaceId;
  const space = await getDeliberationSpaceById(spacePk);

  console.log('space info: ', space);

  const dehydratedState = dehydrate(queryClient);

  return (
    <ClientProviders spaceId={spaceId}>
      <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
    </ClientProviders>
  );
}
