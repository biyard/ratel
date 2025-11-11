import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePoll from '../../../hooks/use-poll';
import { Space } from '@/features/spaces/types/space';
import { ListPollResponse } from '../../../types/list-poll-response';
import { useEffect, useState } from 'react';
import { Poll } from '../../../types/poll';
import { State } from '@/types/state';
import { useCreatePollSpaceMutation } from '../../../hooks/use-create-poll-mutation';
import { useDeletePollSpaceMutation } from '../../../hooks/use-delete-poll-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { NavigateFunction, useNavigate } from 'react-router';
import { route } from '@/route';
import { logger } from '@/lib/logger';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { call } from '@/lib/api/ratel/call';

export class SpacePollsEditorController {
  constructor(
    public navigate: NavigateFunction,
    public spacePk: string,
    public t: TFunction<'SpacePollsEditor', undefined>,
    public space: Space,
    public poll: ListPollResponse,
    public polls: State<Poll[]>,
    public bookmark: State<string | null>,
    public showSelector: State<boolean>,

    public createPoll: ReturnType<typeof useCreatePollSpaceMutation>,
    public deletePoll: ReturnType<typeof useDeletePollSpaceMutation>,
  ) {}

  handleCreatePoll = async (isPrePoll: boolean) => {
    try {
      const v = await this.createPoll.mutateAsync({
        spacePk: this.spacePk,
        default: isPrePoll,
      });

      showSuccessToast('Poll created successfully');
      this.navigate(route.spacePollById(this.spacePk, v.sk));
    } catch (err) {
      logger.error('Failed to create poll', err);
      showErrorToast('Failed to create poll');
    }
  };

  enterPoll = (pollPk: string) => {
    this.navigate(route.spacePollById(this.spacePk, pollPk));
  };

  handleDeletePoll = async (pollSk: string) => {
    try {
      await this.deletePoll.mutateAsync({
        spacePk: this.spacePk,
        pollSk,
      });

      showSuccessToast('Poll deleted successfully');

      // Remove the deleted poll from the local state
      const currentPolls = this.polls.get();
      this.polls.set(currentPolls.filter((p) => p.sk !== pollSk));
    } catch (err) {
      logger.error('Failed to delete poll', err);
      showErrorToast('Failed to delete poll');
    }
  };

  loadMore = async () => {
    const bm = this.bookmark.get();
    if (!bm) return;

    const next = await call(
      'GET',
      `/v3/spaces/${encodeURIComponent(this.spacePk)}/polls?bookmark=${encodeURIComponent(
        bm,
      )}`,
    );

    const page = new ListPollResponse(next);
    const prev = this.polls.get() ?? [];
    this.polls.set([...prev, ...page.polls]);
    this.bookmark.set(page.bookmark ?? null);
  };

  shouldShowPrePoll = (): boolean => {
    return (
      this.polls.get().length === 0 ||
      !this.polls.get().some((poll) => poll.default)
    );
  };
}

export function useSpacePollsEditorController(spacePk: string) {
  const navigate = useNavigate();
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePoll(spacePk);
  const { t } = useTranslation('SpacePollsEditor');
  const polls = useState<Poll[]>(poll.polls || []);
  const bookmark = useState<string | null>(poll.bookmark ?? null);
  const showSelector = useState<boolean>(false);

  const createPoll = useCreatePollSpaceMutation();
  const deletePoll = useDeletePollSpaceMutation();

  useEffect(() => {
    polls[1](poll?.polls ?? []);
    bookmark[1](poll?.bookmark ?? null);
  }, [poll?.bookmark, poll?.polls]);

  return new SpacePollsEditorController(
    navigate,
    spacePk,
    t,
    space,
    poll,
    new State(polls),
    new State(bookmark),
    new State(showSelector),

    createPoll,
    deletePoll,
  );
}
