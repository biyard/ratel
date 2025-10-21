import {
  SurveyAnswer,
  SurveyAnswerType,
  SurveyQuestion,
  SurveyQuestionWithAnswer,
} from '@/features/spaces/polls/types/poll-question';
import ObjectiveViewer from './objective-viewer';
import DropdownViewer from './dropdown-viewer';
import LinearScaleViewer from './linear-scale-viewer';
import SubjectiveViewer from './subjective-viewer';
import React from 'react';
import { I18nFunction } from '../index';
import Card from '@/components/card';

export interface SurveyViewerProps {
  t: I18nFunction;
  questions: SurveyQuestion[];
  selectedAnswers: Record<number, SurveyAnswer>;
  onUpdateAnswer: (questionIdx: number, answer: SurveyAnswer) => void;
  disabled?: boolean;
}
export default function SurveyViewer({
  t,
  disabled,
  questions,
  selectedAnswers,
  onUpdateAnswer,
}: SurveyViewerProps) {
  const questionsWithAnswers = questions.map((q, index) => {
    return {
      answer_type: q.answer_type,
      question: q,
      answer: selectedAnswers[index],
    } as SurveyQuestionWithAnswer;
  });

  return (
    <div className="flex flex-col gap-2.5 w-full">
      {questions.length === 0 && (
        <span className="flex justify-center items-center w-full text-neutral-500">
          {t('no_questions')}
        </span>
      )}
      {questionsWithAnswers.map((questionAnswer, idx) => (
        <Card key={`survey-question-${idx}`}>
          <QuestionViewer
            t={t}
            questionAnswer={questionAnswer}
            disabled={disabled}
            updateAnswer={(answer) => onUpdateAnswer(idx, answer)}
          />
        </Card>
      ))}
    </div>
  );
}

function QuestionViewer({
  t,
  disabled = false,
  questionAnswer,
  updateAnswer,
}: {
  t: I18nFunction;
  disabled?: boolean;
  questionAnswer: SurveyQuestionWithAnswer;
  updateAnswer: (answer: SurveyAnswer) => void;
}): React.JSX.Element | null {
  switch (questionAnswer.answer_type) {
    case SurveyAnswerType.SingleChoice: {
      const { question, answer } = questionAnswer;
      const prevAnswers = answer?.answer !== undefined ? [answer.answer] : [];

      return (
        <ObjectiveViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedIndexes={prevAnswers}
          onSelect={(idx) => {
            let nextAnswer = idx;
            if (prevAnswers.includes(idx)) {
              nextAnswer = undefined;
            }

            updateAnswer({
              answer_type: question.answer_type,
              answer: nextAnswer,
            });
          }}
        />
      );
    }
    case SurveyAnswerType.Checkbox:
    case SurveyAnswerType.MultipleChoice: {
      const { question, answer } = questionAnswer;
      const prevAnswers = answer?.answer ?? [];
      return (
        <ObjectiveViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedIndexes={prevAnswers}
          onSelect={(idx) => {
            const next = prevAnswers.includes(idx)
              ? prevAnswers.filter((i) => i !== idx)
              : [...prevAnswers, idx];
            updateAnswer({
              answer_type: question.answer_type,
              answer: next,
            });
          }}
        />
      );
    }

    case SurveyAnswerType.Dropdown: {
      const { question, answer } = questionAnswer;
      return (
        <DropdownViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedOption={answer?.answer}
          onSelect={(optIndex) => {
            updateAnswer({
              answer_type: question.answer_type,
              answer: optIndex,
            });
          }}
        />
      );
    }

    case SurveyAnswerType.LinearScale: {
      const { question, answer } = questionAnswer;
      return (
        <LinearScaleViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedValue={answer?.answer}
          onSelect={(value) =>
            updateAnswer({
              answer_type: question.answer_type,
              answer: value,
            })
          }
        />
      );
    }

    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective: {
      const { question, answer } = questionAnswer;
      return (
        <SubjectiveViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          inputValue={answer?.answer ?? ''}
          onInputChange={(value) =>
            updateAnswer({
              answer_type: question.answer_type,
              answer: value,
            })
          }
        />
      );
    }

    default:
      return <div></div>;
  }
}
