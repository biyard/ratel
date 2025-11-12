import { useTranslation } from 'react-i18next';
import SurveyViewer from '../survey/viewer';
import usePollSpace from '../../polls/hooks/use-poll-space';
import { SurveyAnswer } from '../../polls/types/poll-question';
import { logger } from '@/lib/logger';
import { useState } from 'react';
import { usePollResponseMutation } from '../../polls/hooks/use-poll-response-mutation';
import { useErrorZone } from '@/features/errors/hooks/use-error-zone';
import { ErrorSpacePollRequiredField } from '@/features/errors/types/errors';

export type PollRequirementProps = React.HTMLAttributes<HTMLDivElement> & {
  spacePk: string;
  pollSk: string;
  onNext?: () => void;
};

export default function PollRequirement({
  spacePk,
  pollSk,
  onNext,
}: PollRequirementProps) {
  const { t } = useTranslation('SpaceSurvey');
  const { data: poll } = usePollSpace(spacePk, pollSk);
  const defaultAnswers: Record<number, SurveyAnswer | null> = {};
  const { setError, removeError, ErrorZone } = useErrorZone();

  poll.questions.forEach((q, idx) => {
    logger.debug(`Question ${idx}`, q);
    defaultAnswers[idx] = null;
  });

  const [answers, setAnswers] =
    useState<Record<number, SurveyAnswer | null>>(defaultAnswers);

  const respondPoll = usePollResponseMutation();
  const handleSubmit = async () => {
    logger.debug('Submitting poll answers', answers);
    // Clear any previous errors
    removeError();

    respondPoll.mutate(
      {
        spacePk,
        pollSk,
        answers: Object.values(answers),
      },
      {
        onSuccess: () => {
          logger.debug('Poll response submitted successfully');
          onNext?.();
        },
        onError: (error) => {
          logger.error('Failed to submit poll response:', error);
          setError(ErrorSpacePollRequiredField);
        },
      },
    );
  };

  const handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );

    answers[questionIdx] = answer;
    setAnswers({ ...answers });
  };

  return (
    <>
      <ErrorZone />
      <SurveyViewer
        t={t}
        questions={poll.questions}
        status={poll.status}
        onUpdateAnswer={handleUpdateAnswer}
        selectedAnswers={answers}
        onValidateError={() => setError(ErrorSpacePollRequiredField)}
        onSubmit={handleSubmit}
        onLogin={() => {}}
        canSubmit={true}
        disabled={respondPoll.isPending}
        canUpdate={false}
        isLogin={true}
      />
    </>
  );
}
