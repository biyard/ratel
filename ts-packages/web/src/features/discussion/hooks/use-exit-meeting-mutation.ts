import { discussionKeys } from '@/constants';
import { discussionExitMeeting } from '@/features/discussion/utils/discussion.v3';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type ExitMeetingProps = {
  spacePk: string;
  discussionPk: string;
};

export function useExitMeetingMutation() {
  const queryClient = useQueryClient();
  let sp = '';
  let dp = '';

  return useMutation({
    mutationFn: async (vars: ExitMeetingProps) => {
      const { spacePk, discussionPk } = vars;
      sp = spacePk;
      dp = discussionPk;

      return discussionExitMeeting(spacePk, discussionPk);
    },

    onSuccess: (response) => {
      logger.debug('exit meeting response: ', response);

      queryClient.invalidateQueries({
        queryKey: discussionKeys.detail(sp, dp),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to exit meeting');
    },
  });
}
