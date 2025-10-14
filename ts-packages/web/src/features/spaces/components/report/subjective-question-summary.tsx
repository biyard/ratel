import {
  BaseSubjectiveSummary,
  SubjectiveQuestionUnion,
} from '@/types/survey-type';
import { I18nFunction } from '.';

interface SubjectiveResponseProps {
  t: I18nFunction;
  question: SubjectiveQuestionUnion;
  summary: BaseSubjectiveSummary;
}
export default function SubjectiveQuestionSummary({
  t,
  question: { title },
  summary: { answers, total_count },
}: SubjectiveResponseProps) {
  return (
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="flex items-center justify-between border-b border-divider pb-2">
        <div className="text-base font-semibold text-neutral-400">{title}</div>
        <div className="text-sm font-medium text-neutral-400">
          {total_count} {t('total_response_count_unit')}
        </div>
      </div>

      <div className="flex flex-col gap-2">
        {Object.entries(answers).map(([answerText, count], index) => (
          <div
            key={index}
            className="px-4 py-2 bg-input-box-bg border border-input-box-border rounded-md text-sm text-text-primary whitespace-pre-wrap"
          >
            {answerText} ({count})
          </div>
        ))}
      </div>
    </div>
  );
}
