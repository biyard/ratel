import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';
import { deleteSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';

export function useDeleteDiscussionMutation<
  T extends SpaceDiscussionResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['delete-discussion'],
    mutationFn: async ({
      spacePk,
      discussionPk,
    }: {
      spacePk: string;
      discussionPk: string;
    }) => {
      await deleteSpaceDiscussion(spacePk, discussionPk);
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.discussions(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        return response;
      });
    },
  });

  return mutation;
}
