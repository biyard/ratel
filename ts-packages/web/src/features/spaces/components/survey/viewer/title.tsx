import { SurveyAnswerType } from '@/features/spaces/polls/types/poll-question';
import { I18nFunction } from '..';

export interface TitleProps {
  t: I18nFunction;
  is_required?: boolean;
  answer_type: SurveyAnswerType;
  is_multi?: boolean;
  title: string;
}
export default function Title({
  t,
  title,
  is_required,
  answer_type,
  is_multi,
}: TitleProps) {
  let label = '';
  if (
    answer_type === SurveyAnswerType.SingleChoice ||
    (answer_type === SurveyAnswerType.Checkbox && !is_multi)
  ) {
    label = t('single_choice_label');
  } else if (answer_type === SurveyAnswerType.Checkbox && is_multi) {
    label = t('multiple_choice_label');
  }

  return (
    <div>
      <div className="flex flex-row flex-wrap gap-1 w-full font-semibold mt-1.75 mb-3.75 text-base/[22.5px] text-text-primary">
        <span
          className={`whitespace-nowrap ${is_required ? 'text-[#ff6467]' : 'text-blue-500'}`}
        >
          [
          {is_required
            ? t('is_required_true_label')
            : t('is_required_false_label')}
          ]
        </span>

        {label && (
          <span className="whitespace-nowrap text-blue-500">[{label}]</span>
        )}
        <span className="wrap-anywhere">{title}</span>
      </div>
    </div>
  );
}
