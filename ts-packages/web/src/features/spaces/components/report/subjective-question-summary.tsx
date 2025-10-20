import {
  BaseSubjectiveSummary,
  SubjectiveQuestionUnion,
} from '@/features/spaces/polls/types/poll-question';
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
    <div className="flex flex-col gap-5 p-5 w-full bg-transparent rounded-xl border border-neutral-500">
      <div className="flex justify-between items-center pb-2 border-b border-divider">
        <div className="text-base font-semibold text-neutral-400">{title}</div>
        <div className="text-sm font-medium text-neutral-400">
          {total_count} {t('total_response_count_unit')}
        </div>
      </div>

      <div className="flex flex-col gap-2">
        {Object.entries(answers).map(([answerText, count], index) => (
          <div
            key={index}
            className="py-2 px-4 text-sm whitespace-pre-wrap rounded-md border bg-input-box-bg border-input-box-border text-text-primary"
          >
            {answerText} ({count})
          </div>
        ))}
      </div>
    </div>
  );
}
