import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { QK_INF_POSTS } from '@/constants';
import { getQueryClient } from '@/providers/getQueryClient';
import { ListPostResponse, listPosts } from '@/lib/api/ratel/posts.v3';

export function getOptions() {
  return {
    queryKey: [QK_INF_POSTS],
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
