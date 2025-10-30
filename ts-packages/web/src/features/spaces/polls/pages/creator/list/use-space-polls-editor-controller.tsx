import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePoll from '../../../hooks/use-poll';
import { Space } from '@/features/spaces/types/space';
import { ListPollResponse } from '../../../types/list-poll-response';
import { useEffect, useState } from 'react';
import { Poll } from '../../../types/poll';
import { State } from '@/types/state';
import { useCreatePollSpaceMutation } from '../../../hooks/use-create-poll-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { NavigateFunction, useNavigate } from 'react-router';
import { route } from '@/route';
import { logger } from '@/lib/logger';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';

export class SpacePollsEditorController {
  constructor(
    public navigate: NavigateFunction,
    public spacePk: string,
    public t: TFunction<'SpacePollsEditor', undefined>,
    public space: Space,
    public poll: ListPollResponse,
    public polls: State<Poll[]>,
    public bookmark: State<string | null>,

    public createPoll: ReturnType<typeof useCreatePollSpaceMutation>,
  ) {}

  handleCreatePoll = async () => {
    try {
      const v = await this.createPoll.mutateAsync({
        spacePk: this.spacePk,
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
}

export function useSpacePollsEditorController(spacePk: string) {
  const navigate = useNavigate();
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePoll(spacePk);
  const { t } = useTranslation('SpacePollsEditor');
  const polls = useState<Poll[]>(poll.polls || []);
  const bookmark = useState<string | null>(poll.bookmark ?? null);

  const createPoll = useCreatePollSpaceMutation();

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

    createPoll,
  );
}
