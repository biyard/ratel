import { TFunction } from 'i18next';
import { SurveyAnswerType, SurveyQuestion } from '@/types/survey-type';
import { DialPad2, Remove } from '@/components/icons';
import { Input } from '@/components/ui/input';

type ObjectiveQuestion = Extract<
  SurveyQuestion,
  { content: { options: string[] } }
>;

interface ObjectiveQuestionEditorProps {
  t: TFunction<'Survey', undefined>;
  question: ObjectiveQuestion;
  onUpdate: (newQuestion: ObjectiveQuestion) => void;
}

export default function ObjectiveQuestionEditor({
  t,
  question,
  onUpdate,
}: ObjectiveQuestionEditorProps) {
  const { answer_type, content } = question;

  const handleUpdateOption = (idx: number, value: string) => {
    const newOptions = [...content.options];
    newOptions[idx] = value;
    onUpdate({ ...question, content: { ...content, options: newOptions } });
  };

  const handleAddOption = () => {
    const newOptions = [...content.options, ''];
    onUpdate({ ...question, content: { ...content, options: newOptions } });
  };

  const handleRemoveOption = (idx: number) => {
    if (content.options.length <= 1) {
      return;
    }
    const newOptions = content.options.filter((_, index) => index !== idx);
    onUpdate({ ...question, content: { ...content, options: newOptions } });
  };

  return (
    <div className="flex flex-col gap-2">
      {content.options.map((opt, idx) => (
        <div key={`option--${idx}`} className="flex gap-2.5 items-center">
          <DialPad2 className="w-6 h-6" />

          {answer_type === SurveyAnswerType.Checkbox && (
            <div className="w-6 h-6 rounded-sm border border-c-wg-50 light:border-[#e5e5e5] max-tablet:hidden" />
          )}

          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            type="text"
            placeholder={t('option_hint')}
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
        + {t('add_option')}
      </button>
    </div>
  );
}
