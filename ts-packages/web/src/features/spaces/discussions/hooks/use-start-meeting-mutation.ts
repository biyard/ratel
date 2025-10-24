import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { SpaceDiscussionResponse } from '../types/space-discussion-response';
import { discussionStartMeeting } from '@/lib/api/ratel/discussion.spaces.v3';

export function useStartMeetingMutation<T extends SpaceDiscussionResponse>() {
  const mutation = useMutation({
    mutationKey: ['start-meeting'],
    mutationFn: async ({
      spacePk,
      discussionPk,
    }: {
      spacePk: string;
      discussionPk: string;
    }) => {
      await discussionStartMeeting(spacePk, discussionPk);
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
