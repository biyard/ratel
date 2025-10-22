import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';
import { createSpaceDiscussion } from '@/lib/api/ratel/discussion.spaces.v3';

export function useCreateDiscussionMutation<
  T extends SpaceDiscussionResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['create-discussion'],
    mutationFn: async ({
      spacePk,
      started_at,
      ended_at,

      name,
      description,
      user_ids,
    }: {
      spacePk: string;
      started_at: number;
      ended_at: number;

      name: string;
      description: string;
      user_ids: string[];
    }) => {
      await createSpaceDiscussion(
        spacePk,
        started_at,
        ended_at,
        name,
        description,
        user_ids,
      );
    },
    onSuccess: async (
      _,
      { spacePk, started_at, ended_at, name, description },
    ) => {
      const spaceQK = spaceKeys.discussions(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (response) => {
        response.started_at = started_at;
        response.ended_at = ended_at;
        response.name = name;
        response.description = description;
        return response;
      });
    },
  });

  return mutation;
}
