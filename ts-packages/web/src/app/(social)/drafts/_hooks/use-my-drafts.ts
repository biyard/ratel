import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { FeedStatus } from '@/features/posts/types/post';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { listMyDrafts } from '@/lib/api/ratel/me.v3';
import { ListPostResponse } from '@/features/posts/dto/list-post-response';

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
  const user = useSuspenseUserInfo();

  const username = user.data?.username || '';

  return useSuspenseInfiniteQuery(getOptions(username));
}
