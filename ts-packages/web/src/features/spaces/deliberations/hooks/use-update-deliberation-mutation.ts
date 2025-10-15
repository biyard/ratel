import { spaceKeys } from '@/constants';
import { NewDiscussionCreateRequest } from '@/lib/api/models/discussion';
import { NewSurveyCreateRequest } from '@/lib/api/models/survey';
import { showErrorToast } from '@/lib/toast';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import {
  BackendFile,
  SpaceVisibility,
  updateDeliberationSpace,
} from '../utils/deliberation.spaces.v3';

type SpaceProps = {
  spacePk: string;
  html_contents: string;
  files: BackendFile[];
  discussions: NewDiscussionCreateRequest[];
  elearning_files: BackendFile[];
  surveys: NewSurveyCreateRequest[];
  recommendation_files: BackendFile[];
  visibility: SpaceVisibility;
  started_at: number;
  ended_at: number;
  title?: string;
  recommendation_html_contents?: string;
};

export function useUpdateDeliberationMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (vars: SpaceProps) => {
      const {
        spacePk,
        html_contents,
        files,
        discussions,
        elearning_files,
        surveys,
        recommendation_files,
        visibility,
        started_at,
        ended_at,
        title,
        recommendation_html_contents,
      } = vars;

      return updateDeliberationSpace(
        spacePk,
        html_contents,
        files,
        discussions,
        elearning_files,
        surveys,
        recommendation_files,
        visibility,
        started_at,
        ended_at,
        title,
        recommendation_html_contents,
      );
    },

    onSuccess: (deliberation) => {
      queryClient.invalidateQueries({
        queryKey: spaceKeys.detail(deliberation.pk),
      });
    },

    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to update space');
    },
  });
}
