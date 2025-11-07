import React, { useState } from 'react';
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
import Card from '@/components/card';
import { I18nFunction } from '../index';
import { Button } from '@/components/ui/button';
import { showErrorToast } from '@/lib/toast';
import { PollStatus } from '@/features/spaces/polls/types/poll-status';

export interface SurveyViewerProps {
  t: I18nFunction;
  questions: SurveyQuestion[];
  selectedAnswers: Record<number, SurveyAnswer>;
  onUpdateAnswer: (questionIdx: number, answer: SurveyAnswer) => void;
  onSubmit?: () => void;
  onLogin?: () => void;
  status: PollStatus;
  isAdmin?: boolean;
  canSubmit?: boolean;
  canUpdate?: boolean;
  isLogin?: boolean;
  disabled?: boolean;
  initialIndex?: number;
}

export default function SurveyViewer({
  t,
  disabled,
  questions,
  status,
  selectedAnswers,
  onUpdateAnswer,
  onSubmit,
  onLogin,
  canSubmit,
  canUpdate,
  isLogin,
  isAdmin = false,
  initialIndex = 0,
}: SurveyViewerProps) {
  let button = <></>;

  const [idx, setIdx] = useState(initialIndex);
  const qa = questions.map((q, index) => {
    return {
      answer_type: q.answer_type,
      question: q,
      answer: selectedAnswers[index],
    } as SurveyQuestionWithAnswer;
  });

  const total = qa.length;
  const current = qa[idx];

  const isValidAnswer = (answer: SurveyAnswer['answer']) => {
    return (
      answer !== undefined &&
      answer !== null &&
      !(Array.isArray(answer) && answer.length === 0) &&
      !(typeof answer === 'string' && answer.trim() === '')
    );
  };

  const canNext = () => {
    if (isAdmin) return true;
    // Allow navigation in view-only mode (when disabled is false but can't submit/update)
    if (disabled === false && !canSubmit && !canUpdate) return true;
    const ans = current.answer?.answer;
    if (current.question.is_required && !isValidAnswer(ans)) return false;
    return true;
  };

  const validateAllRequiredAnswers = () => {
    for (let i = 0; i < qa.length; i++) {
      const question = qa[i];
      if (!question.question.is_required) continue;

      if (!isValidAnswer(question.answer?.answer)) {
        return false;
      }
    }
    return true;
  };

  if (!isLogin) {
    button = <Button onClick={onLogin}>{t('btn_login')}</Button>;
  } else if (idx < total - 1) {
    button = (
      <Button
        onClick={() => {
          if (!isAdmin) {
            if (
              current.question.is_required &&
              !isValidAnswer(current.answer?.answer)
            ) {
              showErrorToast(
                'Please answer this required question before proceeding.',
              );
              return;
            }
          }
          setIdx((v) => Math.min(total - 1, v + 1));
        }}
        disabled={(!canNext() && !isAdmin) || disabled}
      >
        {t('btn_next')}
      </Button>
    );
  } else if (canSubmit && !isAdmin && status == PollStatus.InProgress) {
    button = (
      <Button
        onClick={() => {
          if (!validateAllRequiredAnswers()) {
            showErrorToast(
              'Please answer all required questions before submitting.',
            );
            return;
          }
          onSubmit?.();
        }}
      >
        {t('btn_submit')}
      </Button>
    );
  } else if (canUpdate && !isAdmin) {
    button = (
      <Button
        onClick={() => {
          if (!validateAllRequiredAnswers()) {
            showErrorToast(
              'Please answer all required questions before updating.',
            );
            return;
          }
          onSubmit?.();
        }}
      >
        {t('btn_update')}
      </Button>
    );
  }

  return (
    <div className="flex flex-col gap-3 w-full">
      {total === 0 && (
        <span className="flex justify-center items-center w-full text-neutral-500">
          {t('no_questions')}
        </span>
      )}

      {total > 0 && (
        <>
          <Card key={`survey-question-${idx}`}>
            <div className="flex items-center justify-between">
              <div className="text-sm/[22.5px] text-white font-medium">
                {idx + 1} / {total}
              </div>
            </div>
            <QuestionViewer
              t={t}
              questionAnswer={current}
              disabled={disabled}
              updateAnswer={(answer) => {
                onUpdateAnswer(idx, answer);

                if (!isAdmin) {
                  const type = current.question.answer_type;
                  const isAutoNext =
                    type === SurveyAnswerType.SingleChoice ||
                    type === SurveyAnswerType.Dropdown ||
                    type === SurveyAnswerType.LinearScale ||
                    (type === SurveyAnswerType.Checkbox &&
                      // eslint-disable-next-line @typescript-eslint/no-explicit-any
                      (current.question as any).is_multi === false);

                  const hasValidAnswer =
                    answer?.answer !== undefined &&
                    !(
                      Array.isArray(answer?.answer) &&
                      answer.answer.length === 0
                    );

                  if (isAutoNext && hasValidAnswer && idx < total - 1) {
                    setIdx((v) => Math.min(total - 1, v + 1));
                  }
                }
              }}
            />
          </Card>

          <div className="flex items-center justify-between gap-2">
            {idx != 0 ? (
              <Button
                onClick={() => setIdx((v) => Math.max(0, v - 1))}
                disabled={idx === 0}
              >
                {t('btn_prev')}
              </Button>
            ) : (
              <div></div>
            )}

            {button}
          </div>
        </>
      )}
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
      const prev = answer?.answer !== undefined ? [answer.answer] : [];
      return (
        <ObjectiveViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedIndexes={prev}
          onSelect={(i) => {
            if (disabled) return;
            let next = i;
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            if (prev.includes(i)) next = undefined as any;
            updateAnswer({ answer_type: question.answer_type, answer: next });
          }}
        />
      );
    }
    case SurveyAnswerType.Checkbox:
    case SurveyAnswerType.MultipleChoice: {
      const { question, answer } = questionAnswer;
      const prev = answer?.answer ?? [];
      return (
        <ObjectiveViewer
          t={t}
          {...question}
          answer_type={question.answer_type}
          disabled={disabled}
          selectedIndexes={prev}
          onSelect={(i) => {
            if (disabled) return;
            const next = prev.includes(i)
              ? prev.filter((n: number) => n !== i)
              : [...prev, i];
            updateAnswer({ answer_type: question.answer_type, answer: next });
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
          onSelect={(opt) => {
            if (disabled) return;
            updateAnswer({ answer_type: question.answer_type, answer: opt });
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
          onSelect={(v) => {
            if (disabled) return;
            updateAnswer({ answer_type: question.answer_type, answer: v });
          }}
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
          onInputChange={(v) => {
            if (disabled) return;
            updateAnswer({ answer_type: question.answer_type, answer: v });
          }}
        />
      );
    }
    default:
      return <div />;
  }
}
