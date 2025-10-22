import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';
import { updateSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';

export function useUpdateDiscussionMutation<
  T extends SpaceDiscussionResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['update-discussion'],
    mutationFn: async ({
      spacePk,
      discussionPk,

      started_at,
      ended_at,

      name,
      description,
      user_ids,
    }: {
      spacePk: string;
      discussionPk: string;

      started_at: number;
      ended_at: number;

      name: string;
      description: string;
      user_ids: string[];
    }) => {
      await updateSpaceDiscussion(
        spacePk,
        discussionPk,
        started_at,
        ended_at,
        name,
        description,
        user_ids,
      );
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.discussion(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });
    },
  });

  return mutation;
}
