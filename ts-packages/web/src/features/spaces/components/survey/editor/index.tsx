import { DialPad } from '@/assets/icons/security';
import {
  createDefaultQuestion,
  SurveyAnswerType,
  SurveyQuestion,
} from '@/types/survey-type';
import TypeSelect from './type-select';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import ObjectiveQuestionEditor from './objective-question';
import LinearScaleQuestionEditor from './linear-scale-question';
import { Add } from '@/assets/icons/validations';
import SwitchButton from '@/components/switch-button';
import { I18nFunction } from '..';

interface SurveyEditorProps {
  t: I18nFunction;
  questions: SurveyQuestion[];
  onAddQuestion: () => void;
  onDeleteQuestion: (questionIdx: number) => void;
  onUpdateQuestion: (questionIdx: number, newQuestion: SurveyQuestion) => void;
}

export default function SurveyEditor({
  t,
  questions,
  onAddQuestion,
  onDeleteQuestion,
  onUpdateQuestion,
}: SurveyEditorProps) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      {questions.map((question, index) => (
        <SurveyEditorItem
          key={index}
          t={t}
          question={question}
          onUpdate={(newQuestion) => onUpdateQuestion(index, newQuestion)}
          onDelete={() => onDeleteQuestion(index)}
        />
      ))}
      <div className="relative flex items-center justify-center w-full py-6">
        <div
          className="absolute top-1/2 w-full h-0.25"
          style={{
            borderTop: '1px dashed transparent',
            borderImage:
              'repeating-linear-gradient(to right, #525252 0 8px, transparent 8px 16px) 1',
          }}
        />

        <div
          className="cursor-pointer z-10 bg-background flex items-center justify-center w-fit h-fit p-[13px] border border-neutral-500 rounded-full"
          onClick={onAddQuestion}
        >
          <Add className="w-4 h-4 stroke-neutral-500 text-neutral-500" />
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
    <div className="flex flex-col w-full bg-card-bg-secondary border border-card-border rounded-[10px] px-4 pb-5 pt-1">
      <div className="flex flex-row w-full justify-center items-center mb-2.5">
        <DialPad className="w-6 h-6" />
      </div>
      <div className="flex flex-col w-full gap-2.5">
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

        <div className="flex flex-col mt-2.5 gap-2.5">
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
    <div className="flex flex-row w-full justify-end items-center mt-4">
      <div className="flex flex-wrap w-fit max-tablet:gap-4 max-tablet:justify-end gap-10">
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

        <div
          className="cursor-pointer flex flex-row w-fit gap-1.25 items-center"
          onClick={onDelete}
        >
          <div className="text-[15px] text-neutral-500 font-medium cursor-pointer">
            {t('delete_button_label')}
          </div>
          <Trash2 className="w-4.5 h-4.5 stroke-white light:stroke-neutral-500 cursor-pointer" />
        </div>
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
    <label className="flex items-center cursor-pointer gap-2 select-none">
      <span
        className={`font-medium text-[15px]/[24px] ${value ? textColor : 'text-gray-400'}`}
      >
        {label}
      </span>
      <SwitchButton value={value} onChange={onChange} color={bgColor} />
    </label>
  );
}
