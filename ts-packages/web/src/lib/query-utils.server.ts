import { getCookieContext } from '@/app/_providers/CookieProvider';
import { getOrMakeQueryClient } from '@/providers/getQueryClient';
import { QueryClient } from '@tanstack/react-query';

export async function getServerQueryClient(): Promise<QueryClient> {
  const { nextSession } = await getCookieContext();
  if (!nextSession) {
    throw new Error('No session found in cookies');
  }
  const queryClient = getOrMakeQueryClient(nextSession);

  return queryClient;
}
