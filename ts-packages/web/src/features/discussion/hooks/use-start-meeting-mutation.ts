import { discussionKeys } from '@/constants';
import { discussionStartMeeting } from '@/features/discussion/utils/discussion.v3';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type StartMeetingProps = {
  spacePk: string;
  discussionPk: string;
};

export function useStartMeetingMutation() {
  const queryClient = useQueryClient();
  let sp = '';
  let dp = '';

  return useMutation({
    mutationFn: async (vars: StartMeetingProps) => {
      const { spacePk, discussionPk } = vars;
      sp = spacePk;
      dp = discussionPk;

      return discussionStartMeeting(spacePk, discussionPk);
    },

    onSuccess: (response) => {
      logger.debug('start meeting response: ', response);
      queryClient.invalidateQueries({
        queryKey: discussionKeys.detail(sp, dp),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to start meeting');
    },
  });
}
