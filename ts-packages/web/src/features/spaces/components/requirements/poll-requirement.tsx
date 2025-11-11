import { useTranslation } from 'react-i18next';
import SurveyViewer from '../survey/viewer';
import usePoll from '../../polls/hooks/use-poll';
import usePollSpace from '../../polls/hooks/use-poll-space';
import { SurveyAnswer } from '../../polls/types/poll-question';
import { logger } from '@/lib/logger';
import { useState } from 'react';

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
  const handleSubmit = () => {
    onNext();
  };
  const [answers, setAnswers] = useState<Record<number, SurveyAnswer>>(
    poll?.myResponse.reduce(
      (acc, answer, idx) => {
        acc[idx] = answer;
        return acc;
      },
      {} as Record<number, SurveyAnswer>,
    ) || {},
  );

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
      <SurveyViewer
        t={t}
        questions={poll.questions}
        status={poll.status}
        onUpdateAnswer={handleUpdateAnswer}
        selectedAnswers={answers}
        onSubmit={handleSubmit}
        onLogin={() => {}}
        canSubmit={true}
        disabled={false}
        canUpdate={false}
        isLogin={true}
      />
    </>
  );
}
