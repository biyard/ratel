import { feedKeys } from '@/constants';
import { reply } from '@/lib/api/ratel/comments.v3';
import { getQueryClient } from '@/providers/getQueryClient';
import { useMutation } from '@tanstack/react-query';
import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';
import { optimisticUpdate } from '@/lib/hook-utils';

export function useReplyCommentMutation() {
  return useMutation({
    mutationFn: async ({
      postPk,
      commentSk,
      content,
    }: {
      postPk: string;
      commentSk: string;
      content: string;
    }) => {
      const resp = await reply(postPk, commentSk, content);

      return { postPk, commentSk, comment: resp };
    },
    onSuccess: async ({ postPk, commentSk }) => {
      const queryKey = feedKeys.repliesOfComment(postPk, commentSk);
      const queryClient = getQueryClient();
      queryClient.invalidateQueries({ queryKey });

      // Update the parent comment's replies count and the post's comments count
      const postQueryKey = feedKeys.detail(postPk);
      await optimisticUpdate<PostDetailResponse>(
        { queryKey: postQueryKey },
        (post) => {
          // Increment the post's comments count (replies are also comments)
          post.post.comments = post.post.comments + 1;

          // Increment the parent comment's replies count
          post.comments = post.comments.map((comment) => {
            if (comment.sk === commentSk) {
              return {
                ...comment,
                replies: comment.replies + 1,
              };
            }
            return comment;
          });

          return post;
        },
      );
    },
  });
}
