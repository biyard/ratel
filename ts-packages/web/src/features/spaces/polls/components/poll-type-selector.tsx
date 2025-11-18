import Card from '@/components/card';
import { TFunction } from 'i18next';

export type PollTypeSelectorProps = {
  t: TFunction<'SpacePollsEditor', undefined>;
  onSelectType: (isPrePoll: boolean) => void;
  showPrePoll?: boolean;
};

export function PollTypeSelector({
  t,
  onSelectType,
  showPrePoll,
}: PollTypeSelectorProps) {
  return (
    <div className="flex flex-col gap-4 w-full">
      {/* Pre-poll Survey Card */}
      {showPrePoll && (
        <Card
          className="transition-colors cursor-pointer hover:bg-card-bg-secondary/80"
          data-testid="create-pre-poll-survey"
          onClick={() => onSelectType(true)}
        >
          <div className="flex flex-col gap-2">
            <div className="flex gap-2 items-center">
              <span className="text-2xl">📋</span>
              <h3 className="text-lg font-semibold text-text-primary">
                {t('pre_poll_survey_title')}
              </h3>
            </div>
            <p className="text-sm text-text-secondary">
              {t('pre_poll_survey_description')}
            </p>
          </div>
        </Card>
      )}

      {/* Final Survey Card */}
      <Card
        className="transition-colors cursor-pointer hover:bg-card-bg-secondary/80"
        data-testid="create-final-survey"
        onClick={() => onSelectType(false)}
      >
        <div className="flex flex-col gap-2">
          <div className="flex gap-2 items-center">
            <span className="text-2xl">✅</span>
            <h3 className="text-lg font-semibold text-text-primary">
              {t('final_survey_title')}
            </h3>
          </div>
          <p className="text-sm text-text-secondary">
            {t('final_survey_description')}
          </p>
        </div>
      </Card>
    </div>
  );
}
