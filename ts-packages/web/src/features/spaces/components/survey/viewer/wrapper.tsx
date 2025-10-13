import { SurveyAnswerType } from '@/types/survey-type';
import { TFunction } from 'i18next';

export interface WrapperProps {
  t: TFunction<'Survey', undefined>;
  is_required?: boolean;
  answer_type: SurveyAnswerType;
  is_multi?: boolean;
  title: string;
}
export default function Wrapper({
  t,
  title,
  is_required,
  answer_type,
  is_multi,
}: WrapperProps) {
  let label = '';
  if (
    answer_type === SurveyAnswerType.SingleChoice ||
    (answer_type === SurveyAnswerType.Checkbox && !is_multi)
  ) {
    label = t('single_choice');
  } else if (answer_type === SurveyAnswerType.Checkbox && is_multi) {
    label = t('multiple_choice');
  }

  return (
    <div>
      <div className="flex flex-row w-full mt-1.75 mb-3.75 font-semibold text-base/[22.5px] text-text-primary gap-1">
        <div className={is_required ? 'text-[#ff6467]' : 'text-blue-500'}>
          [{is_required ? t('required') : t('optional')}]
        </div>

        {label && <div className="text-blue-500">[{label}]</div>}

        <div>{title}</div>
      </div>
    </div>
  );
}
