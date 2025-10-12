import { showErrorToast } from '@/components/custom-toast/toast';
import { feedKeys } from '@/constants';
import { likeComment } from '@/lib/api/ratel/comments.v3';
import { ListResponse } from '@/lib/api/ratel/common';
import { PostComment, PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import { logger } from '@/lib/logger';
import { useMutation } from '@tanstack/react-query';

export function useLikeCommentMutation() {
  return useMutation({
    mutationFn: async ({
      postPk,
      commentSk,
      like,
    }: {
      postPk: string;
      commentSk: string;
      like: boolean;
    }) => {
      const resp = await likeComment(postPk, commentSk, like);

      return { postPk, commentSk, resp };
    },

    onMutate: async ({ postPk, commentSk, like }) => {
      const queryKey = feedKeys.detail(postPk);
      const repliesQueryKey = feedKeys.repliesOfComment(postPk, commentSk);

      let backupPost = await optimisticUpdate<PostDetailResponse>(
        { queryKey },
        (post) => {
          const comments = post.comments.map((c) => {
            logger.debug('Comment in list', c, commentSk);
            if (c.sk === commentSk) {
              if (c.liked === like) {
                return c;
              }

              if (like) {
                return {
                  ...c,
                  liked: like,
                  likes: c.likes + 1,
                };
              } else {
                return {
                  ...c,
                  liked: like,
                  likes: Math.max(0, c.likes - 1),
                };
              }
            }

            return c;
          });

          return { ...post, comments };
        },
      );

      let backupReplies = await optimisticListUpdate<ListResponse<PostComment>>(
        { queryKey: repliesQueryKey },
        (replies) => {
          const items = replies.items.map((c) => {
            if (c.sk === commentSk) {
              return {
                ...c,
                liked: like,
                likes: c.likes + 1,
              };
            }

            return c;
          });

          return { ...replies, items };
        },
      );

      return { backupPost, backupReplies };
    },
    onError: (error: Error, _variables, context) => {
      logger.error('Failed to like comment', error);
      context?.backupPost?.rollback();
      context?.backupReplies?.rollback();

      showErrorToast(error.message || 'Failed to like comment');
    },
  });
}
