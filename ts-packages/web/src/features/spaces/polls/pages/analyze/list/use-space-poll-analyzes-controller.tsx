import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePoll from '../../../hooks/use-poll';
import { Space } from '@/features/spaces/types/space';
import { ListPollResponse } from '../../../types/list-poll-response';
import { useEffect, useState } from 'react';
import { Poll } from '../../../types/poll';
import { State } from '@/types/state';
import { NavigateFunction, useNavigate } from 'react-router';
import { route } from '@/route';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { call } from '@/lib/api/ratel/call';

export class SpacePollAnalyzesController {
  constructor(
    public navigate: NavigateFunction,
    public spacePk: string,
    public t: TFunction<'SpacePollsEditor', undefined>,
    public space: Space,
    public poll: ListPollResponse,
    public polls: State<Poll[]>,
    public bookmark: State<string | null>,
  ) {}

  enterPoll = (pollPk: string) => {
    this.navigate(route.spaceAnalyzePollById(this.spacePk, pollPk));
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
}

export function useSpacePollAnalyzesController(spacePk: string) {
  const navigate = useNavigate();
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePoll(spacePk);
  const { t } = useTranslation('SpacePollsEditor');
  const polls = useState<Poll[]>(poll.polls || []);
  const bookmark = useState<string | null>(poll.bookmark ?? null);

  useEffect(() => {
    polls[1](poll?.polls ?? []);
    bookmark[1](poll?.bookmark ?? null);
  }, [poll?.bookmark, poll?.polls]);

  return new SpacePollAnalyzesController(
    navigate,
    spacePk,
    t,
    space,
    poll,
    new State(polls),
    new State(bookmark),
  );
}
