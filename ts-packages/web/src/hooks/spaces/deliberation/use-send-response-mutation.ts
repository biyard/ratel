import { spaceKeys } from '@/constants';
import { Answer } from '@/lib/api/models/response';
import { updateDeliberationResponseSpace } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';

type SpaceResponseProps = {
  spacePk: string;
  survey_pk: string;
  answers: Answer[];
};

export function useSendDeliberationResponseMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (vars: SpaceResponseProps) => {
      const { spacePk, survey_pk, answers } = vars;

      return updateDeliberationResponseSpace(spacePk, survey_pk, answers);
    },

    onSuccess: (response) => {
      queryClient.invalidateQueries({
        queryKey: spaceKeys.detail(response.pk),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to update space');
    },
  });
}
