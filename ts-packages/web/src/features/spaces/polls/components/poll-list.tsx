import { Poll } from '../types/poll';
import { TFunction } from 'i18next';
import { PollItem } from './poll-item';

export type PollListProps = {
  polls: Poll[];
  bookmark: string | null | undefined;
  t: TFunction<'SpacePollsEditor', undefined>;
  enterPoll?: (pollPk: string) => void;
  deletePoll?: (pollSk: string) => void;
  loadMore?: () => void;
  isAnalyze?: boolean;
};

export function PollList({
  polls,
  t,
  bookmark,
  enterPoll,
  deletePoll,
  loadMore,
  isAnalyze,
}: PollListProps) {
  const hasMore = !!bookmark;

  return (
    <div className="flex flex-col gap-5 w-full">
      <div className="flex flex-col gap-2.5 w-full">
        {polls.map(
          (poll) =>
            (!isAnalyze || poll.questions.length > 0) && (
              <PollItem
                key={poll.sk}
                poll={poll}
                name={`${poll.default ? t('sample_survey') : t('final_survey')}`}
                enterPoll={enterPoll}
                deletePoll={deletePoll}
                label={isAnalyze ? t('view_analyze') : t('enter')}
              />
            ),
        )}

        {hasMore && (
          <button
            className="self-center py-2 px-4 mt-2 rounded-md border border-divider hover:bg-white/5"
            onClick={loadMore}
          >
            {t('more')}
          </button>
        )}
      </div>
    </div>
  );
}
