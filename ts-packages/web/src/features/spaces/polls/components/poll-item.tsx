import { Button } from '@/components/ui/button';
import Card from '@/components/card';
import { Poll } from '../types/poll';
import { useTranslation } from 'react-i18next';

export type PollItemProps = {
  poll: Poll;
  enterPoll?: (pollPk: string) => void;
  deletePoll?: (pollSk: string) => void;
  label: string;
  name: string;
};

export function PollItem({
  poll,
  enterPoll,
  deletePoll,
  label,
  name,
}: PollItemProps) {
  const { t } = useTranslation('SpacePollsEditor');
  return (
    <Card
      key={poll.sk}
      className="flex flex-row justify-between items-center w-full"
    >
      <div className="flex flex-col gap-1 w-full">
        <div className="font-semibold text-[12px] text-neutral-300 leading-[20px]">{`${poll.questions.length} ${t('questions')}`}</div>
        <div className="text-base font-medium text-text-primary leading-[20px]">
          {name}
        </div>
      </div>

      <div className="flex gap-2">
        {deletePoll && (
          <Button
            variant="outline"
            className="w-fit"
            onClick={() => deletePoll(poll.sk)}
          >
            {t('delete')}
          </Button>
        )}
        <Button
          className="w-fit light:bg-neutral-300"
          onClick={() => enterPoll?.(poll.sk)}
        >
          {label}
        </Button>
      </div>
    </Card>
  );
}
