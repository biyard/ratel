import { getCookieContext } from '@/app/_providers/CookieProvider';
import { makeQueryClient } from '@/providers/getQueryClient';
import { QueryClient } from '@tanstack/react-query';
import { cache } from 'react';

const getQueryClient = cache(() => makeQueryClient());

export async function getServerQueryClient(): Promise<QueryClient> {
  const { nextSession } = await getCookieContext();
  if (!nextSession) {
    throw new Error('No session found in cookies');
  }

  return getQueryClient();
}
