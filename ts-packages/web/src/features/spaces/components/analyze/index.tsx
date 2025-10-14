import { logger } from '@/lib/logger';

import {
  SurveyAnswerType,
  SurveyQuestion,
  SurveySummary,
} from '@/types/survey-type';
import { TFunction } from 'i18next';
import SummaryReport from './report';
import SubjectiveQuestionSummary from './subjective-question-summary';
import ObjectiveQuestionSummary from './objective-question-summary';
import { useTranslation } from 'react-i18next';

const handleDownloadExcel = (summaries: SurveySummary[]) => {
  logger.debug('Download Excel clicked with summaries: ', summaries);
  // FIXME: Implement download excel for Survey
};

export type I18nFunction = TFunction<'Analyze', undefined>;
export interface AnalyzeProps {
  startedAt: number;
  endedAt: number;
  totalResponses: number;
  questions: SurveyQuestion[];
  summaries: SurveySummary[];
}
export function Analyze({
  startedAt,
  endedAt,
  totalResponses,
  questions,
  summaries,
}: AnalyzeProps) {
  const { t } = useTranslation('Analyze');

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full justify-end mb-[20px]">
        <div className="w-fit">
          <button
            className="w-full px-[20px] py-[10px] rounded-[10px] bg-[#fcb300] hover:bg-[#ca8f00] text-black text-bold text-[16px] hover:text-black cursor-pointer"
            disabled={false}
            onClick={() => {
              handleDownloadExcel(summaries);
            }}
          >
            {t('download_excel')}
          </button>
        </div>
      </div>

      <div className="flex flex-col w-full gap-2.5">
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
  question: SurveyQuestion;
  summary: SurveySummary;
}

export default function SummaryItem({
  t,
  question,
  summary,
}: SummaryItemProps) {
  if (
    (summary.type === SurveyAnswerType.Subjective &&
      question.answer_type === SurveyAnswerType.Subjective) ||
    (summary.type === SurveyAnswerType.ShortAnswer &&
      question.answer_type === SurveyAnswerType.ShortAnswer)
  ) {
    return (
      <SubjectiveQuestionSummary t={t} question={question} summary={summary} />
    );
  }

  if (
    summary.type !== SurveyAnswerType.Subjective &&
    summary.type !== SurveyAnswerType.ShortAnswer &&
    question.answer_type !== SurveyAnswerType.Subjective &&
    question.answer_type !== SurveyAnswerType.ShortAnswer
  ) {
    return (
      <ObjectiveQuestionSummary t={t} question={question} summary={summary} />
    );
  }

  return null;
}
