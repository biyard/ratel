import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';
import { discussionParticipateMeeting } from '@/lib/api/ratel/discussion.spaces.v3';

export function useParticipateMeetingMutation<
  T extends SpaceDiscussionResponse,
>() {
  const mutation = useMutation({
    mutationKey: ['participate-meeting'],
    mutationFn: async ({
      spacePk,
      discussionPk,
    }: {
      spacePk: string;
      discussionPk: string;
    }) => {
      await discussionParticipateMeeting(spacePk, discussionPk);
    },
    onSuccess: async (_, { spacePk, discussionPk }) => {
      const discussionQk = spaceKeys.discussion(spacePk, discussionPk);
      await optimisticUpdate<T>({ queryKey: discussionQk }, (response) => {
        return response;
      });
    },
  });

  return mutation;
}
