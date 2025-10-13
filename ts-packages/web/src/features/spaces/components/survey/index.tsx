import { SurveyAnswer, SurveyQuestion } from '@/types/survey-type';
import SurveyViewer from './viewer';
import SurveyEditor from './editor';
import { useTranslation } from 'react-i18next';

export interface SpaceSurveyProps {
  isEditMode: boolean;
  questions: SurveyQuestion[];
  answers: SurveyAnswer[];
  handleAddQuestion: () => void;
  handleUpdateQuestion: (index: number, question: SurveyQuestion) => void;
  handleDeleteQuestion: (index: number) => void;
  handleUpdateAnswer: (questionIdx: number, answer: SurveyAnswer) => void;
}
export default function SpaceSurvey({
  isEditMode,
  questions,
  answers,
  handleAddQuestion,
  handleUpdateQuestion,
  handleDeleteQuestion,
  handleUpdateAnswer,
}: SpaceSurveyProps) {
  const { t } = useTranslation('Survey');
  return (
    <div className="flex flex-col w-full">
      {isEditMode ? (
        <SurveyEditor
          t={t}
          questions={questions}
          onAddQuestion={handleAddQuestion}
          onUpdateQuestion={handleUpdateQuestion}
          onDeleteQuestion={handleDeleteQuestion}
        />
      ) : (
        <SurveyViewer
          questions={questions}
          selectedAnswers={answers}
          updateAnswer={handleUpdateAnswer}
        />
      )}
    </div>
  );
}
