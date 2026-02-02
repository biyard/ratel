import { useTranslation } from 'react-i18next';
import usePollSpace from '../../polls/hooks/use-poll-space';
import { SurveyAnswer } from '../../polls/types/poll-question';
import { logger } from '@/lib/logger';
import { useEffect, useState } from 'react';
import { usePollResponseMutation } from '../../polls/hooks/use-poll-response-mutation';
import { useErrorZone } from '@/features/errors/hooks/use-error-zone';
import { ErrorSpacePollRequiredField } from '@/features/errors/types/errors';
import { Error } from '@/features/errors/types/errors';
import { usePopup } from '@/lib/contexts/popup-service';
import CompleteSurveyPopup from '../../polls/components/modal/complete_survey';
import { useSpaceById } from '../../hooks/use-space-by-id';
import { SpaceType } from '../../types/space-type';
import { SpaceStatus } from '../../types/space-common';
import SurveyViewer from '../../components/survey/viewer';
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
  const { t: modalT } = useTranslation('SpaceCompleteSurvey');
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollSk);
  const popup = usePopup();
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
    removeError();

    if (!poll) {
      logger.error('Poll data is not available');
      return;
    }

    const defaultAnswerByType = (
      answer_type: SurveyAnswer['answer_type'],
    ): SurveyAnswer => {
      switch (answer_type) {
        case 'single_choice':
        case 'dropdown':
        case 'linear_scale':
        case 'short_answer':
        case 'subjective':
          return { answer_type, answer: null };
        case 'multiple_choice':
        case 'checkbox':
          return { answer_type, answer: [] };
        default:
          return { answer_type, answer: null };
      }
    };

    const total = poll.questions.length;

    const payload: SurveyAnswer[] = Array.from({ length: total }, (_, i) => {
      const existing = answers[i];
      if (existing !== undefined && existing !== null) {
        return existing;
      }

      const q = poll.questions[i];
      return defaultAnswerByType(q.answer_type as SurveyAnswer['answer_type']);
    });

    respondPoll.mutate(
      {
        spacePk,
        pollSk,
        answers: payload,
      },
      {
        onSuccess: () => {
          popup
            .open(
              <CompleteSurveyPopup
                onConfirm={() => {
                  onNext?.();
                  popup.close();
                }}
              />,
            )
            .withTitle(modalT('modal_title'))
            .withoutClose();

          logger.debug('Poll response submitted successfully');
        },
        onError: (error) => {
          logger.error('Failed to submit poll response:', error);
          setError(ErrorSpacePollRequiredField);
        },
      },
    );
  };
  const canParticipate = space
    ? space.isAdmin() ||
      space.spaceType !== SpaceType.Deliberation ||
      space.participated
    : undefined;
  useEffect(() => {
    if (canParticipate === false) {
      setError(new Error('space.poll.cannot_participate'));
    } else if (canParticipate === true) {
      removeError();
    }
  }, [canParticipate, setError, removeError]);
  const handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );

    answers[questionIdx] = answer;
    setAnswers({ ...answers });

    logger.debug('Updated answers state:', answers);
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
        onValidateError={() => {
          logger.error('Validation error in poll response', answers);
          setError(ErrorSpacePollRequiredField);
        }}
        onSubmit={handleSubmit}
        onLogin={() => {}}
        canParticipate={!!canParticipate}
        canSubmit={space.status !== SpaceStatus.Finished}
        disabled={respondPoll.isPending || canParticipate === false}
        canUpdate={false}
        isLogin={true}
        isFinished={
          space.status === SpaceStatus.Finished || poll.ended_at < Date.now()
        }
      />
    </>
  );
}
