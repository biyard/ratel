import { SurveyAnswer, SurveyQuestion } from '@/types/survey-type';
import SurveyViewer from './viewer';
import SurveyEditor from './editor';
import { useTranslation } from 'react-i18next';

import { TFunction } from 'i18next';

export type I18nFunction = TFunction<'SpaceSurvey', undefined>;

export interface SurveyProps {
  isEditMode: boolean;

  questions: SurveyQuestion[];
  onAddQuestion: () => void;
  onUpdateQuestion: (index: number, question: SurveyQuestion) => void;
  onDeleteQuestion: (index: number) => void;

  answers: Record<number, SurveyAnswer>;
  onUpdateAnswer: (questionIdx: number, answer: SurveyAnswer) => void;
  isSurveyProgress?: boolean;
}
export default function Survey({
  isEditMode,
  questions,
  onAddQuestion,
  onUpdateQuestion,
  onDeleteQuestion,

  answers,
  onUpdateAnswer,

  isSurveyProgress,
}: SurveyProps) {
  const { t } = useTranslation('SpaceSurvey');
  return (
    <div className="flex flex-col w-full">
      {isEditMode ? (
        <SurveyEditor
          t={t}
          questions={questions}
          onAddQuestion={onAddQuestion}
          onUpdateQuestion={onUpdateQuestion}
          onDeleteQuestion={onDeleteQuestion}
        />
      ) : (
        <SurveyViewer
          t={t}
          questions={questions}
          selectedAnswers={answers}
          onUpdateAnswer={onUpdateAnswer}
          disabled={!isSurveyProgress}
        />
      )}
    </div>
  );
}
