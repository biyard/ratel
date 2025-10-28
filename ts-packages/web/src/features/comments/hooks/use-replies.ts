import { feedKeys } from '@/constants';
import PostComment from '@/features/posts/types/post-comment';
import { listReplies } from '@/lib/api/ratel/comments.v3';
import { ListResponse } from '@/lib/api/ratel/common';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';

export function useReplies(postPk: string, commentSk: string) {
  return useSuspenseInfiniteQuery({
    queryKey: feedKeys.repliesOfComment(postPk, commentSk),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListResponse<PostComment>> =>
      listReplies(postPk, commentSk, pageParam),
    getNextPageParam: (last: ListResponse<PostComment>) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: true,
  });
}
