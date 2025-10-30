import { useMutation } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { optimisticListUpdate, optimisticUpdate } from '@/lib/hook-utils';
import PostResponse from '../dto/list-post-response';
import { PostDetailResponse } from '../dto/post-detail-response';
import Post from '../types/post';

export type CreatePostResponse = {
  post_pk: string;
};

export function createPost(team_pk?: string): Promise<CreatePostResponse> {
  if (team_pk) {
    return call('POST', '/v3/posts', { team_pk });
  }
  return call('POST', '/v3/posts');
}

export function useCreatePostMutation() {
  const { data: user } = useSuspenseUserInfo();
  const createMutation = useMutation({
    mutationFn: ({ teamPk }: { teamPk?: string }) => createPost(teamPk),

    onSuccess: async ({ post_pk }) => {
      const queryKey = feedKeys.detail(post_pk);
      const listQueryKey = feedKeys.drafts(user.username!);
      const draft: Partial<Post> = {
        pk: post_pk,
        user_pk: user.pk,
        author_display_name: user.nickname,
        author_username: user.username,
        author_profile_url: user.profile_url,
        author_type: user.user_type,
      };
      const rollbackDraft = await optimisticUpdate<PostDetailResponse>(
        { queryKey },
        (prev) => {
          return {
            ...prev!,
            post: {
              ...prev!.post,
              ...draft,
            },
          };
        },
      );

      const rollbackDrafts = await optimisticListUpdate<PostResponse>(
        { queryKey: listQueryKey },
        (post) => {
          if (post.pk !== post_pk) return post;

          return {
            ...post!,
            ...draft,
          };
        },
      );

      return { rollbackDraft, rollbackDrafts };
    },
    onError: (error: Error) => {
      throw new Error(error.message || 'Failed to create draft');
    },
  });

  return createMutation;
}

// export function useCreateTeamPostMutation() {
//   const queryClient = getQueryClient();
//   const createMutation = useMutation({
//     mutationFn: ({ teamPk }: { teamPk?: string }) => createPost(teamPk),
//     onSuccess: (newDraft) => {
//       const queryKey = feedKeys.detail(postPk);

//       queryClient.setQueryData(feedKeys.detail(newDraft.post_pk), newDraft);
//     },
//     onError: (error: Error) => {
//       throw new Error(error.message || 'Failed to create draft');
//     },
//   });

//   return createMutation;
// }
