import { Button } from '@/components/ui/button';
import { Poll } from '../types/poll';
import Card from '@/components/card';
import { TFunction } from 'i18next';

export type PollListProps = {
  canEdit?: boolean;
  polls: Poll[];
  t: TFunction<'SpacePollsEditor', undefined>;
  createPoll?: () => void;
  enterPoll?: (pollPk: string) => void;
};

export function PollList({
  canEdit,
  polls,
  t,
  createPoll,
  enterPoll,
}: PollListProps) {
  return (
    <div className="flex flex-col w-full gap-5">
      {canEdit && (
        <div className="flex flex-row w-full justify-end">
          <Button variant="primary" className="w-[120px]" onClick={createPoll}>
            {t('create_poll')}
          </Button>
        </div>
      )}

      <div className="flex flex-col w-full gap-2.5">
        {polls.map((poll) => (
          <Card className="flex flex-row w-full justify-between items-center">
            <div className="flex flex-col w-full gap-1">
              <div className="text-[12px] font-semibold text-neutral-300 leading-[20px]">{`${poll.questions.length} ${t('questions')}`}</div>
              <div className="text-base font-medium text-text-primary leading-[20px]">{`${poll.response_editable ? t('sample_survey') : t('final_survey')}`}</div>
            </div>

            <Button
              className="w-[80px] light:bg-neutral-300"
              onClick={() => enterPoll?.(poll.sk)}
            >
              {t('enter')}
            </Button>
          </Card>
        ))}
      </div>
    </div>
  );
}
