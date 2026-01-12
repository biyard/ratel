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
import { useMemo, useState } from 'react';
import SummaryOption, { Primary } from './summary-option';

export type I18nFunction = TFunction<'SpaceSurveyReport', undefined>;

export interface ReportProps {
  startedAt: number;
  endedAt: number;
  totalResponses: number;
  editable: boolean;
  questions: PollQuestion[];
  summaries: SurveySummary[];
  summariesByGender?: Record<string, SurveySummary[]>;
  summariesByAge?: Record<string, SurveySummary[]>;
  summariesBySchool?: Record<string, SurveySummary[]>;
  handleDownloadExcel: () => void;
}

export default function Report({
  startedAt,
  endedAt,
  totalResponses,
  editable,
  questions,
  summaries,
  summariesByGender,
  summariesByAge,
  summariesBySchool,
  handleDownloadExcel,
}: ReportProps) {
  const { t } = useTranslation('SpaceSurveyReport');

  const [primary, setPrimary] = useState<Primary>('overall');
  const [detailKey, setDetailKey] = useState<string | null>(null);

  const displaySummaries = useMemo(() => {
    if (primary === 'overall' || !detailKey) return summaries;
    if (primary === 'gender')
      return summariesByGender?.[detailKey] ?? summaries;
    if (primary === 'age') return summariesByAge?.[detailKey] ?? summaries;
    if (primary === 'school')
      return summariesBySchool?.[detailKey] ?? summaries;
    return summaries;
  }, [
    primary,
    detailKey,
    summaries,
    summariesByGender,
    summariesByAge,
    summariesBySchool,
  ]);

  return (
    <div className="flex flex-col w-full">
      {!editable && (
        <div className="flex justify-end mb-5">
          <Button variant="rounded_primary" onClick={handleDownloadExcel}>
            {t('download_excel_button_label')}
          </Button>
        </div>
      )}

      {totalResponses > 0 && (
        <SummaryReport
          t={t}
          startedAt={startedAt}
          endedAt={endedAt}
          totalResponses={totalResponses}
        />
      )}

      {totalResponses > 0 && (
        <SummaryOption
          t={t}
          primary={primary}
          setPrimary={setPrimary}
          detailKey={detailKey}
          setDetailKey={setDetailKey}
          summariesByGender={summariesByGender}
          summariesByAge={summariesByAge}
          summariesBySchool={summariesBySchool}
        />
      )}

      {totalResponses > 0 && (
        <div className="flex flex-col gap-2.5 w-full">
          {displaySummaries.map((summary, idx) => (
            <SummaryItem
              key={`summary-item-${idx}`}
              t={t}
              question={questions[idx]}
              summary={summary}
            />
          ))}
        </div>
      )}

      {totalResponses <= 0 && <div>{t('not_found_response')}</div>}
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
