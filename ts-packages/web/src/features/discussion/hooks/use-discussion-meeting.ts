import { discussionKeys } from '@/constants';
import { getMeetingByDiscussionId } from '@/features/discussion/utils/discussion.v3';
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

      return getMeetingByDiscussionId(spacePk, discussionPk);
    },

    onSuccess: (response) => {
      logger.debug('discussion meeting response: ', response);

      queryClient.invalidateQueries({
        queryKey: discussionKeys.meeting(sp, dp),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to query discussion meeting');
    },
  });
}
