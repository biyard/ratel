import { feedKeys } from '@/constants';
import { comment } from '@/lib/api/ratel/comments.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { PostDetailResponse } from '../dto/post-detail-response';

export function useCommentMutation() {
  return useMutation({
    mutationFn: async ({
      postPk,
      content,
    }: {
      postPk: string;
      content: string;
    }) => {
      const resp = await comment(postPk, content);

      return { postPk, content, comment: resp };
    },
    onSuccess: async ({ postPk, comment }) => {
      const queryKey = feedKeys.detail(postPk);

      await optimisticUpdate<PostDetailResponse>({ queryKey }, (post) => {
        post.comments = [comment, ...post.comments];

        return post;
      });
    },
  });
}
