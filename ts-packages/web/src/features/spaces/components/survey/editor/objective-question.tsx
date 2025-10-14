import {
  LinearScaleQuestionType,
  ObjectiveQuestionUnion,
  SurveyAnswerType,
} from '@/types/survey-type';
import { DialPad2, Remove } from '@/components/icons';
import { Input } from '@/components/ui/input';
import { I18nFunction } from '..';

export type ObjectiveQuestionWithOptions = Exclude<
  ObjectiveQuestionUnion,
  LinearScaleQuestionType
>;
interface ObjectiveQuestionEditorProps {
  t: I18nFunction;
  question: ObjectiveQuestionWithOptions;
  onUpdate: (newQuestion: ObjectiveQuestionWithOptions) => void;
}

export default function ObjectiveQuestionEditor({
  t,
  question,
  onUpdate,
}: ObjectiveQuestionEditorProps) {
  const handleUpdateOption = (idx: number, value: string) => {
    const newOptions = [...question.options];
    newOptions[idx] = value;
    onUpdate({ ...question, options: newOptions });
  };

  const handleAddOption = () => {
    const newOptions = [...question.options, ''];
    onUpdate({ ...question, options: newOptions });
  };

  const handleRemoveOption = (idx: number) => {
    if (question.options.length <= 1) {
      return;
    }
    const newOptions = question.options.filter((_, index) => index !== idx);
    onUpdate({ ...question, options: newOptions });
  };

  return (
    <div className="flex flex-col gap-2">
      {question.options.map((opt, idx) => (
        <div key={`option--${idx}`} className="flex gap-2.5 items-center">
          <DialPad2 className="w-6 h-6" />

          {question.answer_type === SurveyAnswerType.Checkbox && (
            <div className="w-6 h-6 rounded-sm border border-c-wg-50 light:border-[#e5e5e5] max-tablet:hidden" />
          )}

          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            type="text"
            placeholder={t('option_input_placeholder')}
            value={opt}
            onChange={(e) => handleUpdateOption(idx, e.target.value)}
          />
          <Remove
            className="cursor-pointer w-5 h-5 stroke-neutral-400 text-neutral-400"
            onClick={() => handleRemoveOption(idx)}
          />
        </div>
      ))}
      <button
        onClick={handleAddOption}
        className="cursor-pointer text-sm text-neutral-500 font-semibold text-left mt-2"
      >
        + {t('add_option_button_label')}
      </button>
    </div>
  );
}
