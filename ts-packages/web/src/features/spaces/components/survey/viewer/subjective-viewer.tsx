import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import Title, { type TitleProps } from './title';
import { SubjectiveQuestion, SurveyAnswerType } from '@/features/spaces/polls/types/poll-question';

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
          className="py-3 px-4 text-base rounded-lg border focus:border-yellow-500 focus:outline-none bg-input-box-bg border-input-box-border text-text-primary placeholder:text-neutral-600"
          value={inputValue}
          onChange={(e) => onInputChange(e.target.value)}
          disabled={disabled}
        />
      ) : (
        <Textarea
          rows={7}
          placeholder={t('subjective_input_placeholder')}
          className="py-3 px-4 text-base rounded-lg border focus:border-yellow-500 focus:outline-none bg-input-box-bg border-input-box-border min-h-[185px] text-text-primary placeholder:text-neutral-600"
          value={inputValue}
          onChange={(e) => onInputChange(e.target.value)}
          disabled={disabled}
        />
      )}
    </div>
  );
}
