import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import type { ListPostResponse } from '@/lib/api/ratel/posts.v3';
import { FeedStatus } from '@/lib/api/models/feeds';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { listMyPosts } from '@/lib/api/ratel/me.v3';

export function getOptions(username: string) {
  return {
    queryKey: feedKeys.list({ username, status: FeedStatus.Published }),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => listMyPosts(pageParam),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteMyPosts() {
  const { data } = useSuspenseUserInfo();

  const username = data?.username || '';

  return useSuspenseInfiniteQuery(getOptions(username));
}
