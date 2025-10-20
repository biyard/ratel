import { logger } from '@/lib/logger';
import { useSpacePollEditorController } from './use-space-poll-editor-controller';
import { SpacePollPathProps } from '../space-poll-path-props';
import SurveyEditor from '@/features/spaces/components/survey/editor';
import { useTranslation } from 'react-i18next';
import Survey from '@/features/spaces/components/survey';

export function SpacePollEditorPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollEditorPage: spacePk=${spacePk}, pollPk=${pollPk}`);
  const ctrl = useSpacePollEditorController(spacePk, pollPk);
  const { t } = useTranslation('SpaceSurvey');

  return (
    <>
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
    </>
  );
}
