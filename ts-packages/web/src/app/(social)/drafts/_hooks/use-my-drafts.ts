import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { ListPostResponse } from '@/lib/api/ratel/posts.v3';
import { FeedStatus } from '@/lib/api/models/feeds';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { listMyDrafts } from '@/lib/api/ratel/me.v3';

export function getOptions(username: string) {
  return {
    queryKey: feedKeys.list({ username, status: FeedStatus.Draft }),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => listMyDrafts(pageParam),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteMyDrafts() {
  const {
    data: { username },
  } = useSuspenseUserInfo();

  return useSuspenseInfiniteQuery(getOptions(username));
}
