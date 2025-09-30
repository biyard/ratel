import { makeQueryClient } from '@/providers/getQueryClient';
import { QueryClient } from '@tanstack/react-query';
import { cache } from 'react';

const getQueryClient = cache(() => makeQueryClient());

export async function getServerQueryClient(): Promise<QueryClient> {
  return getQueryClient();
}
