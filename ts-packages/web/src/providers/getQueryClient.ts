import { logger } from '@/lib/logger';
import {
  defaultShouldDehydrateQuery,
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

const serverClients = new Map<string, QueryClient>();

export function getOrMakeQueryClient(session: string) {
  // FIXME: implement session cleanup to prevent memory leaks
  if (serverClients.has(session)) {
    return serverClients.get(session)!;
  }

  const queryClient = makeQueryClient();
  serverClients.set(session, queryClient);

  return queryClient;
}

let browserQueryClient: QueryClient | undefined = undefined;

export function getQueryClient(): QueryClient {
  if (isServer) {
    // Server: always make a new query client
    return makeQueryClient();
  } else {
    // Browser: make a new query client if we don't already have one
    // This is very important, so we don't re-make a new client if React
    // suspends during the initial render. This may not be needed if we
    // have a suspense boundary BELOW the creation of the query client
    if (!browserQueryClient) browserQueryClient = makeQueryClient();
    return browserQueryClient;
  }
}

interface InitDataOptions<TData = unknown> {
  key: unknown[];

  data: TData;
}

export function initData(cli: QueryClient, options: InitDataOptions[]) {
  for (const { key, data } of options) {
    cli.setQueryData(key, data);
  }
}
