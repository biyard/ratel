import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { listSpaceReplies } from '@/lib/api/ratel/comments.v3';
import { ListResponse } from '@/lib/api/ratel/common';
import PostComment from '@/features/posts/types/post-comment';

export function useSpaceReplies(
  spacePk: string,
  postPk: string,
  commentSk: string,
) {
  const query = useSuspenseInfiniteQuery({
    queryKey: spaceKeys.boards_replies(spacePk, postPk, commentSk),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListResponse<PostComment>> =>
      listSpaceReplies(spacePk, postPk, commentSk, pageParam),
    getNextPageParam: (last: ListResponse<PostComment>) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  });

  const replies = query.data?.pages.flatMap((page) => page.items ?? []) ?? [];

  return {
    ...query,
    replies,
  };
}
