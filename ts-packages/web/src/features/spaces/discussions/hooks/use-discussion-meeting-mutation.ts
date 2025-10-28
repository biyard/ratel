import { spaceKeys } from '@/constants';
import { getMeetingByDiscussionPk } from '@/lib/api/ratel/discussion.spaces.v3';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type DiscussionMeetingProps = {
  spacePk: string;
  discussionPk: string;
};

export function useDiscussionMeetingMutation() {
  const queryClient = useQueryClient();
  let sp = '';
  let dp = '';

  return useMutation({
    mutationFn: async (vars: DiscussionMeetingProps) => {
      const { spacePk, discussionPk } = vars;
      sp = spacePk;
      dp = discussionPk;

      return getMeetingByDiscussionPk(spacePk, discussionPk);
    },

    onSuccess: (response) => {
      logger.debug('discussion meeting response: ', response);

      queryClient.invalidateQueries({
        queryKey: spaceKeys.discussion_meeting(sp, dp),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to query discussion meeting');
    },
  });
}
