import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { getQueryClient } from '@/providers/getQueryClient';
import { ListPostResponse, listPosts } from '@/lib/api/ratel/posts.v3';
import { FeedStatus } from '@/lib/api/models/feeds';

export function getOptions() {
  return {
    // TODO: v3 API doesn't filter by status on the backend yet
    queryKey: feedKeys.list({ status: FeedStatus.Published }),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => listPosts(pageParam),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export async function prefetchInfiniteFeeds() {
  const queryClient = getQueryClient();
  const options = getOptions();
  await queryClient.prefetchInfiniteQuery(options);
}

export default function useInfiniteFeeds() {
  return useSuspenseInfiniteQuery(getOptions());
}
