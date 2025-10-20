import { logger } from '@/lib/logger';
import { useSpacePollEditorController } from './use-space-poll-editor-controller';
import { SpacePollPathProps } from '../space-poll-path-props';

export function SpacePollEditorPage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(`SpacePollEditorPage: spacePk=${spacePk}, pollPk=${pollPk}`);
  // TODO: use or define hooks
  const _ctrl = useSpacePollEditorController();

  return (
    <>
      <div className="flex flex-col gap-2.5 w-full">
        {questions.map((question, index) => (
          <SurveyEditorItem
            key={index}
            t={t}
            question={question}
            onUpdate={(newQuestion) => onUpdateQuestion(index, newQuestion)}
            onDelete={() => onDeleteQuestion(index)}
          />
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
            onClick={onAddQuestion}
          >
            <Add className="w-4 h-4 stroke-neutral-500 text-neutral-500" />
          </div>
        </div>
      </div>
    </>
  );
}
