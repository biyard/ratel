import { Add } from '@/assets/icons/validations';
import SwitchButton from '@/components/switch-button';
import { Input } from '@/components/ui/input';
import { Separator } from '@/components/ui/separator';
import { SpacePollEditorController } from '@/features/spaces/polls/pages/creator/use-space-poll-editor-controller';
import {
  createDefaultQuestion,
  SurveyAnswerType,
  SurveyQuestion,
} from '@/features/spaces/polls/types/poll-question';
import { Trash2 } from 'lucide-react';
import { I18nFunction } from '..';
import LinearScaleQuestionEditor from './linear-scale-question';
import ObjectiveQuestionEditor from './objective-question';
import TypeSelect from './type-select';

interface SurveyEditorProps {
  ctrl: SpacePollEditorController;
}

export default function SurveyEditor({ ctrl }: SurveyEditorProps) {
  return (
    <div className="flex flex-col gap-2.5 w-full">
      {ctrl.questions.get().map((question, index) => (
        <>
          {index !== 0 && <Separator className="bg-gray-600" />}

          <SurveyEditorItem
            key={`survey-editor-item-${index}`}
            t={ctrl.t}
            question={question}
            onUpdate={(newQuestion) =>
              ctrl.handleUpdateQuestion(index, newQuestion)
            }
            onDelete={() => ctrl.handleRemoveQuestion(index)}
          />
        </>
      ))}
      <div className="flex relative justify-center items-center py-6 w-full">
        <div
          className="absolute top-1/2 w-full h-0.25"
          style={{
            borderTop: '1px dashed transparent',
            borderImage:
              'repeating-linear-gradient(to right, #525252 0 8px, transparent 8px 16px) 1',
          }}
        />

        <div
          className="flex z-10 justify-center items-center rounded-full border cursor-pointer bg-background w-fit h-fit p-[13px] border-neutral-500"
          onClick={ctrl.handleAddQuestion}
        >
          <Add className="w-4 h-4 [&>path]:stroke-neutral-500 stroke-neutral-500 text-neutral-500" />
        </div>
      </div>
    </div>
  );
}

function QuestionContent({
  t,
  question,
  onUpdate,
}: {
  t: I18nFunction;
  question: SurveyQuestion;
  onUpdate: (newQuestion: SurveyQuestion) => void;
}) {
  switch (question.answer_type) {
    case SurveyAnswerType.SingleChoice:
    case SurveyAnswerType.MultipleChoice:
    case SurveyAnswerType.Checkbox:
    case SurveyAnswerType.Dropdown:
      return (
        <ObjectiveQuestionEditor
          t={t}
          question={question}
          onUpdate={onUpdate}
        />
      );

    case SurveyAnswerType.LinearScale:
      return (
        <LinearScaleQuestionEditor
          t={t}
          question={question}
          onUpdate={onUpdate}
        />
      );

    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective:
    default:
      return null;
  }
}

export function SurveyEditorItem({
  t,
  question,
  onUpdate,
  onDelete,
}: {
  t: I18nFunction;
  question: SurveyQuestion;
  onUpdate: (newQuestion: SurveyQuestion) => void;
  onDelete: () => void;
}) {
  const handleTypeChange = (newType: SurveyAnswerType) => {
    const newQuestion = createDefaultQuestion(newType);
    onUpdate(newQuestion);
  };

  const handleTitleChange = (value: string) => {
    const next_question = question;
    next_question.title = value;
    onUpdate(next_question);
  };

  return (
    <div className="flex flex-col px-4 pt-1 pb-5 w-full border bg-card-bg-secondary border-card-border rounded-[10px]">
      <div className="flex flex-col gap-2.5 w-full">
        <div className="flex gap-2 max-tablet:flex-col">
          <TypeSelect
            t={t}
            value={question.answer_type}
            onChange={handleTypeChange}
          />
          <Input
            className="bg-input-box-bg border border-input-box-border rounded-lg w-full px-4 !py-5.5 font-medium text-[15px]/[22.5px] text-text-primary placeholder:text-neutral-600 "
            type="text"
            placeholder={t('question_title_placeholder')}
            value={question.title}
            onChange={(e) => handleTitleChange(e.target.value)}
          />
        </div>

        <div className="flex flex-col gap-2.5 mt-2.5">
          <QuestionContent t={t} question={question} onUpdate={onUpdate} />
        </div>

        <QuestionFooter
          t={t}
          question={question}
          onUpdate={onUpdate}
          onDelete={onDelete}
        />
      </div>
    </div>
  );
}

interface QuestionFooterProps {
  t: I18nFunction;
  question: SurveyQuestion;
  onUpdate: (newQuestion: SurveyQuestion) => void;
  onDelete: () => void;
}

export function QuestionFooter({
  t,
  question,
  onUpdate,
  onDelete,
}: QuestionFooterProps) {
  const handleRequiredChange = (value: boolean) => {
    const next_question = question;
    next_question.is_required = value;
    onUpdate(next_question);
  };

  const handleMultiChange = (value: boolean) => {
    if (question.answer_type === SurveyAnswerType.Checkbox) {
      const next_question = question;
      next_question.is_multi = value;
      onUpdate(next_question);
    }
  };

  return (
    <div className="flex flex-row justify-between items-center mt-4 w-full max-tablet:flex-col max-tablet:items-end max-tablet:gap-4">
      <div className="flex flex-wrap gap-10 w-fit max-tablet:gap-4">
        {question.answer_type === SurveyAnswerType.Checkbox && (
          <LabelSwitchButton
            bgColor="bg-blue-500"
            textColor="text-blue-500"
            label={t('multiple_choice_label')}
            value={question.is_multi ?? false}
            onChange={handleMultiChange}
          />
        )}

        <LabelSwitchButton
          bgColor="bg-red-500"
          textColor="text-red-500"
          label={t('is_required_true_label')}
          value={question.is_required ?? false}
          onChange={handleRequiredChange}
        />
      </div>

      <div
        className="flex flex-row items-center cursor-pointer gap-1.25 px-2 py-1 hover:bg-neutral-800 light:hover:bg-neutral-200 rounded transition-colors"
        onClick={onDelete}
      >
        <span className="font-medium text-[15px] text-neutral-500">
          {t('delete_button_label')}
        </span>
        <Trash2 className="w-4.5 h-4.5 stroke-white light:stroke-neutral-500" />
      </div>
    </div>
  );
}

function LabelSwitchButton({
  label,
  bgColor,
  textColor,
  value,
  onChange,
}: {
  label: string;
  bgColor: string;
  textColor: string;
  value: boolean;
  onChange: (val: boolean) => void;
}) {
  return (
    <label className="flex gap-2 items-center cursor-pointer select-none">
      <span
        className={`font-medium text-[15px]/[24px] ${value ? textColor : 'text-gray-400'}`}
      >
        {label}
      </span>
      <SwitchButton value={value} onChange={onChange} color={bgColor} />
    </label>
  );
}
