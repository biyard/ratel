import { useMutation } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export type CreatePostResponse = {
  post_pk: string;
};

export function createPost(team_pk?: string): Promise<CreatePostResponse> {
  if (team_pk) {
    return call('POST', '/v3/posts', { team_pk });
  }
  return call('POST', '/v3/posts');
}

// TODO: Update to use v3 feed query keys without userId parameter
export function useCreatePostMutation() {
  const queryClient = getQueryClient();
  const createMutation = useMutation({
    mutationFn: ({ teamPk }: { teamPk?: string }) => createPost(teamPk),
    onSuccess: (newDraft) => {
      queryClient.setQueryData(feedKeys.detail(newDraft.post_pk), newDraft);
    },
    onError: (error: Error) => {
      throw new Error(error.message || 'Failed to create draft');
    },
  });

  return createMutation;
}
