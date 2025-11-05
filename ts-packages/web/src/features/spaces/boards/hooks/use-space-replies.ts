import { spaceKeys } from '@/constants';
import PostComment from '@/features/posts/types/post-comment';
import { listSpaceReplies } from '@/lib/api/ratel/comments.v3';
import { ListResponse } from '@/lib/api/ratel/common';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';

export function useSpaceReplies(
  spacePk: string,
  postPk: string,
  commentSk: string,
) {
  return useSuspenseInfiniteQuery({
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
    refetchOnWindowFocus: true,
  });
}
