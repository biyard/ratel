import { logger } from '@/lib/logger';
import {
  defaultShouldDehydrateQuery,
  type InfiniteData,
  isServer,
  QueryClient,
} from '@tanstack/react-query';

export function makeQueryClient() {
  return new QueryClient({
    defaultOptions: {
      mutations: {
        onError(error) {
          logger.error('Query mutation error:', error);
        },
      },
      dehydrate: {
        // include pending queries in dehydration
        shouldDehydrateQuery: (query) =>
          defaultShouldDehydrateQuery(query) ||
          query.state.status === 'pending',
      },

      queries: {
        // With SSR, we usually want to set some default staleTime
        // above 0 to avoid refetching immediately on the client
        staleTime: 60 * 1000,
        retry: 3,
        retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
      },
    },
  });
}

let browserQueryClient: QueryClient | undefined = undefined;

export function getQueryClient(): QueryClient {
  if (!browserQueryClient) browserQueryClient = makeQueryClient();
  return browserQueryClient;
}

export interface InitDataOptions {
  key: unknown[];

  data: unknown | InfiniteData<unknown>;
}

export function initData(cli: QueryClient, options: InitDataOptions[]) {
  for (const { key, data } of options) {
    if (!key || !data) continue;

    cli.setQueryData(key, data);
  }
}
