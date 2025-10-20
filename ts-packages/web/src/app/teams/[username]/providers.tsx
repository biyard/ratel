'use client';

import { ReactNode } from 'react';
import ClientProviders from './providers.client';

import { useTeamDetailByUsername } from '@/features/teams/hooks/use-team';
import { logger } from '@/lib/logger';

export default function Provider({
  children,
  username,
}: {
  children: ReactNode;
  username: string;
}) {
  logger.debug('Provider: username parameter received:', username);

  // Use v3 client-side data fetching - this eliminates all server-side API calls
  const {
    data: teamDetail,
    isLoading,
    error,
  } = useTeamDetailByUsername(username);

  logger.debug('Provider: teamDetail', teamDetail);

  if (isLoading) {
    return <div>Loading team...</div>;
  }

  if (error || !teamDetail) {
    logger.error('Provider: Failed to load team', error);
    return <div>Failed to load team</div>;
  }

  // Extract user ID for context (you may need to get this from a different hook)
  const userId = 0; // TODO: Get from user context/hook

  return <ClientProviders userId={userId}>{children}</ClientProviders>;
}
