import { DialPad } from '@/assets/icons/security';
import { SurveyAnswerType, SurveyQuestion } from '@/types/survey-type';
import { TFunction } from 'i18next';
import TypeSelect from './type-select';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import ObjectiveQuestionEditor from './objective-question';
import LinearScaleQuestionEditor from './linear-scale-question';
import LabelSwitchButton from './label-switch-button';
import { Add } from '@/assets/icons/validations';

function createDefaultQuestion(type: SurveyAnswerType): SurveyQuestion {
  switch (type) {
    case SurveyAnswerType.SingleChoice:
    case SurveyAnswerType.MultipleChoice:
      return {
        answer_type: type,
        content: {
          title: '',
          options: [''],
          is_required: false,
        },
      };
    case SurveyAnswerType.Checkbox:
      return {
        answer_type: type,
        content: {
          title: '',
          description: '',
          options: [''],
          is_multi: false,
          is_required: false,
        },
      };
    case SurveyAnswerType.Dropdown:
      return {
        answer_type: type,
        content: {
          title: '',
          is_required: false,
          options: [''],
        },
      };
    case SurveyAnswerType.LinearScale:
      return {
        answer_type: type,
        content: {
          title: '',
          min_value: 1,
          max_value: 5,
          min_label: '',
          max_label: '',
          is_required: false,
        },
      };
    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective:
      return {
        answer_type: type,
        content: {
          title: '',
          description: '',
          is_required: false,
        },
      };
    default:
      throw new Error(`Unsupported answer type: ${type}`);
  }
}

interface SurveyEditorProps {
  t: TFunction<'Survey', undefined>;
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
  t: TFunction<'Survey', undefined>;
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
  t: TFunction<'Survey', undefined>;
  question: SurveyQuestion;
  onUpdate: (newQuestion: SurveyQuestion) => void;
  onDelete: () => void;
}) {
  const handleTypeChange = (newType: SurveyAnswerType) => {
    const newQuestion = createDefaultQuestion(newType);
    onUpdate(newQuestion);
  };

  const handleTitleChange = (value: string) => {
    onUpdate({
      ...question,
      content: { ...question.content, title: value } as any,
    });
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
            placeholder={t('title_hint')}
            value={question.content.title}
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
  t: TFunction<'Survey', undefined>;
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
    onUpdate({
      ...question,
      content: { ...question.content, is_required: value } as any,
    });
  };

  const handleMultiChange = (value: boolean) => {
    if (question.answer_type === SurveyAnswerType.Checkbox) {
      onUpdate({
        ...question,
        content: { ...question.content, is_multi: value },
      });
    }
  };

  return (
    <div className="flex flex-row w-full justify-end items-center mt-4">
      <div className="flex flex-wrap w-fit max-tablet:gap-4 max-tablet:justify-end gap-10">
        {question.answer_type === SurveyAnswerType.Checkbox && (
          <LabelSwitchButton
            bgColor="bg-blue-500"
            textColor="text-blue-500"
            label={t('multiple_selection')}
            value={question.content.is_multi ?? false}
            onChange={handleMultiChange}
          />
        )}

        <LabelSwitchButton
          bgColor="bg-red-500"
          textColor="text-red-500"
          label={t('required')}
          value={question.content.is_required ?? false}
          onChange={handleRequiredChange}
        />

        <div
          className="cursor-pointer flex flex-row w-fit gap-1.25 items-center"
          onClick={onDelete}
        >
          <div className="text-[15px] text-neutral-500 font-medium cursor-pointer">
            {t('delete')}
          </div>
          <Trash2 className="w-4.5 h-4.5 stroke-white light:stroke-neutral-500 cursor-pointer" />
        </div>
      </div>
    </div>
  );
}
