import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import Title, { type TitleProps } from './title';
import { SubjectiveQuestion, SurveyAnswerType } from '@/types/survey-type';

interface SubjectiveViewerProps extends SubjectiveQuestion, TitleProps {
  inputValue: string;
  disabled?: boolean;

  onInputChange: (value: string) => void;
}
export default function SubjectiveViewer(props: SubjectiveViewerProps) {
  const { t, answer_type, inputValue, onInputChange, disabled } = props;

  return (
    <div className="flex flex-col w-full gap-[10px]">
      <Title {...props} />
      {answer_type === SurveyAnswerType.ShortAnswer ? (
        <Input
          type="text"
          placeholder={t('subjective_input_placeholder')}
          className=" bg-input-box-bg border border-input-box-border text-base text-text-primary placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={inputValue}
          onChange={(e) => onInputChange(e.target.value)}
          disabled={disabled}
        />
      ) : (
        <Textarea
          rows={7}
          placeholder={t('subjective_input_placeholder')}
          className="bg-input-box-bg border border-input-box-border min-h-[185px]  text-base text-text-primary placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={inputValue}
          onChange={(e) => onInputChange(e.target.value)}
          disabled={disabled}
        />
      )}
    </div>
  );
}
