import { SurveyAnswer } from '@/app/spaces/[id]/type';
import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';
import { Answer, surveyResponseCreateRequest } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import { ratelApi } from '@/lib/api/ratel_api';
import { getQueryClient } from '@/providers/getQueryClient';
import { useMutation } from '@tanstack/react-query';
import { getQueryKey } from './use-space-by-id';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export async function responseAnswer(
  spaceId: number,
  surveyId: number,
  answers: Answer[],
): Promise<void> {
  await apiFetch(
    `${config.api_url}${ratelApi.responses.respond_answer(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(surveyResponseCreateRequest(answers, 1, surveyId)),
    },
  );
}

export function usePollMutation() {
  const queryClient = getQueryClient();

  const mutation = useMutation({
    mutationFn: async ({
      spaceId,
      surveyId,
      questions,
      answer,
    }: {
      spaceId: number;
      surveyId: number;
      questions: Question[];
      answer: SurveyAnswer;
    }) => {
      const processedAnswers = questions.map((question, index) => {
        const submittedAnswer = answer.answers[index];

        if (question.is_required && !submittedAnswer) {
          throw new Error('All required fields must be filled.');
        }

        if (submittedAnswer) {
          return submittedAnswer;
        }

        return {
          answer_type: question.answer_type,
          answer: null,
        };
      });

      await responseAnswer(spaceId, surveyId, processedAnswers);
      return spaceId;
    },
    onSuccess: (spaceId) => {
      queryClient.invalidateQueries({
        queryKey: getQueryKey(spaceId),
      });

      showSuccessToast('Poll response submitted successfully');
    },
    onError: (error) => {
      showErrorToast(error.message || 'Failed to submit poll response');
    },
  });

  return mutation;
}
