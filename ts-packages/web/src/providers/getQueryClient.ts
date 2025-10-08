import { logger } from '@/lib/logger';
import {
  defaultShouldDehydrateQuery,
  type InfiniteData,
  QueryClient,
} from '@tanstack/react-query';

type BootData = {
  react_query?: { key: unknown; data: unknown /* updatedAt?: number */ }[];
};

export function makeQueryClient() {
  const queryClient = new QueryClient({
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

  // Hydrate from booststrap json
  const el = document.getElementById('__BOOTSTRAP_DATA__');
  if (el?.textContent) {
    try {
      const boot = JSON.parse(el.textContent) as BootData;
      logger.debug('loaded bootstrap data', boot);
      boot.react_query?.forEach(({ key, data }) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        queryClient.setQueryData(key as any, data);
      });
    } catch (e) {
      logger.error('hydration data parsing error', e);
    }
  }

  return queryClient;
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
