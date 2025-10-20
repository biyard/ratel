import { logger } from '@/lib/logger';

import {
  SurveyAnswerType,
  PollQuestion,
  SurveySummary,
} from '@/features/spaces/polls/types/poll-question';
import { TFunction } from 'i18next';
import SummaryReport from './report';
import SubjectiveQuestionSummary from './subjective-question-summary';
import ObjectiveQuestionSummary from './objective-question-summary';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/button';

const handleDownloadExcel = (summaries: SurveySummary[]) => {
  logger.debug('Download Excel clicked with summaries: ', summaries);
  // FIXME: Implement download excel for Survey
};

export type I18nFunction = TFunction<'SpaceSurveyReport', undefined>;
export interface ReportProps {
  startedAt: number;
  endedAt: number;
  totalResponses: number;
  questions: PollQuestion[];
  summaries: SurveySummary[];
}
export default function Report({
  startedAt,
  endedAt,
  totalResponses,
  questions,
  summaries,
}: ReportProps) {
  const { t } = useTranslation('SpaceSurveyReport');
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row justify-end w-full mb-[20px]">
        <div className="w-fit">
          <Button
            variant="rounded_primary"
            onClick={() => {
              handleDownloadExcel(summaries);
            }}
          >
            {t('download_excel_button_label')}
          </Button>
        </div>
      </div>

      <div className="flex flex-col gap-2.5 w-full">
        {totalResponses > 0 && (
          <SummaryReport
            t={t}
            startedAt={startedAt}
            endedAt={endedAt}
            totalResponses={totalResponses}
          />
        )}
        {summaries.map((summary, index) => (
          <SummaryItem
            key={`summary-item-${index}`}
            t={t}
            question={questions[index]}
            summary={summary}
          />
        ))}
      </div>
    </div>
  );
}

interface SummaryItemProps {
  t: I18nFunction;
  question: PollQuestion;
  summary: SurveySummary;
}

function SummaryItem({ t, question, summary }: SummaryItemProps) {
  if (
    (summary.answer_type === SurveyAnswerType.Subjective &&
      question.answer_type === SurveyAnswerType.Subjective) ||
    (summary.answer_type === SurveyAnswerType.ShortAnswer &&
      question.answer_type === SurveyAnswerType.ShortAnswer)
  ) {
    return (
      <SubjectiveQuestionSummary t={t} question={question} summary={summary} />
    );
  }

  if (
    summary.answer_type !== SurveyAnswerType.Subjective &&
    summary.answer_type !== SurveyAnswerType.ShortAnswer &&
    question.answer_type !== SurveyAnswerType.Subjective &&
    question.answer_type !== SurveyAnswerType.ShortAnswer
  ) {
    return (
      <ObjectiveQuestionSummary t={t} question={question} summary={summary} />
    );
  }

  return null;
}
