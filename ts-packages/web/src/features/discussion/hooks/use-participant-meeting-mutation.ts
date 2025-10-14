import { discussionKeys } from '@/constants';
import { discussionParticipantMeeting } from '@/features/discussion/utils/discussion.v3';
import { logger } from '@/lib/logger';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type ParticipantMeetingProps = {
  spacePk: string;
  discussionPk: string;
};

export function useParticipantMeetingMutation() {
  const queryClient = useQueryClient();
  let sp = '';
  let dp = '';

  return useMutation({
    mutationFn: async (vars: ParticipantMeetingProps) => {
      const { spacePk, discussionPk } = vars;
      sp = spacePk;
      dp = discussionPk;

      return discussionParticipantMeeting(spacePk, discussionPk);
    },

    onSuccess: (response) => {
      logger.debug('participant meeting response: ', response);
      queryClient.invalidateQueries({
        queryKey: discussionKeys.detail(sp, dp),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to participant meeting');
    },
  });
}
