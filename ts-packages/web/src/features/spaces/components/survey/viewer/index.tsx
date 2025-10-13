import {
  SurveyAnswer,
  SurveyAnswerType,
  SurveyQuestion,
} from '@/types/survey-type';
import ObjectiveViewer from './objective-viewer';
import { useTranslation } from 'react-i18next';
import DropdownViewer from './dropdown-viewer';
import LinearScaleViewer from './linear-scale-viewer';
import SubjectiveViewer from './subjective-viewer';
import React from 'react';

export interface SurveyViewerProps {
  questions: SurveyQuestion[];
  selectedAnswers: SurveyAnswer[];
  updateAnswer: (questionIdx: number, answer: SurveyAnswer) => void;
}
export default function SurveyViewer({
  questions,
  selectedAnswers,
  updateAnswer,
}: SurveyViewerProps) {
  return (
    <div>
      {questions.map((question, idx) => (
        <QuestionViewer
          key={`survey-question-${idx}`}
          question={question}
          answer={selectedAnswers[idx]}
          disabled={false}
          updateAnswer={(answer) => updateAnswer(idx, answer)}
        />
      ))}
    </div>
  );
}

function QuestionViewer({
  disabled = false,
  question,
  answer,
  updateAnswer,
}: {
  disabled?: boolean;
  question: SurveyQuestion;
  answer: SurveyAnswer;
  updateAnswer: (answer: SurveyAnswer) => void;
}): React.JSX.Element | null {
  const { t } = useTranslation('PollSpace');

  const updateAnswerHandler = (
    type: SurveyAnswerType,
    currentAnswer: SurveyAnswer,
  ) => {
    if (type === SurveyAnswerType.SingleChoice) {
      return (index: number) => {
        updateAnswer({
          answer_type: SurveyAnswerType.SingleChoice,
          answer: index,
        });
      };
    } else if (
      type === SurveyAnswerType.MultipleChoice ||
      type === SurveyAnswerType.Checkbox
    ) {
      return (index: number) => {
        const prev = Array.isArray(currentAnswer.answer)
          ? (currentAnswer.answer as number[])
          : [];
        const next = prev.includes(index)
          ? prev.filter((i) => i !== index)
          : [...prev, index];
        updateAnswer({
          answer_type: type,
          answer: next,
        });
      };
    } else if (type === SurveyAnswerType.Dropdown) {
      return (optIndex: number) => {
        updateAnswer({
          answer_type: SurveyAnswerType.Dropdown,
          answer: optIndex,
        });
      };
    } else if (type === SurveyAnswerType.LinearScale) {
      return (value: number) => {
        updateAnswer({
          answer_type: SurveyAnswerType.LinearScale,
          answer: value,
        });
      };
    } else if (
      type === SurveyAnswerType.ShortAnswer ||
      type === SurveyAnswerType.Subjective
    ) {
      return (value: string) => {
        updateAnswer({
          answer_type: type,
          answer: value,
        });
      };
    }
    return () => {};
  };

  const handleSelect = updateAnswerHandler(question.answer_type, answer);

  switch (question.answer_type) {
    case SurveyAnswerType.Checkbox:
    case SurveyAnswerType.MultipleChoice:
    case SurveyAnswerType.SingleChoice:
      return (
        <ObjectiveViewer
          t={t}
          {...question.content}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedIndexes={answer.answer as number[]}
          onSelect={handleSelect as (index: number) => void}
        />
      );
    case SurveyAnswerType.Dropdown:
      return (
        <DropdownViewer
          t={t}
          {...question.content}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedOption={
            answer.answer !== undefined ? (answer.answer as number) : null
          }
          onSelect={handleSelect as (optIndex: number) => void}
        />
      );
    case SurveyAnswerType.LinearScale:
      return (
        <LinearScaleViewer
          t={t}
          {...question.content}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedValue={answer.answer as number}
          onSelect={handleSelect as (value: number) => void}
        />
      );
    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective:
      return (
        <SubjectiveViewer
          t={t}
          {...question.content}
          answer_type={question.answer_type}
          disabled={disabled}
          inputValue={answer.answer as string}
          onInputChange={handleSelect as (value: string) => void}
        />
      );
    default:
      return <div></div>;
  }
}
