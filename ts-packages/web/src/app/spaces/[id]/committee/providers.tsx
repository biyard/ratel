import { ReactNode } from 'react';
import ClientProviders from './providers.client';
import { getRedeemCode, getSpaceById } from '@/lib/api/ratel_api.server';
import { getQueryClient, initData } from '@/providers/getQueryClient';

export default async function Provider({
  children,
  spaceId,
}: {
  children: ReactNode;
  spaceId: number;
}) {
  const queryClient = getQueryClient();

  try {
    const space = await getSpaceById(spaceId);
    const redeemCode = await getRedeemCode(spaceId);

    // Initialize the query client with the space data
    initData(queryClient, [space, redeemCode]);
  } catch (error) {
    console.error(
      `Failed to fetch space or redeem code for spaceId ${spaceId}:`,
      error,
    );
    throw error;
  }

  return <ClientProviders spaceId={spaceId}>{children}</ClientProviders>;
}
