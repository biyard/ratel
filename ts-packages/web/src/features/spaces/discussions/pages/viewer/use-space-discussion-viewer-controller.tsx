import { State } from '@/types/state';
import useDiscussionSpace from '../../hooks/use-discussion-space';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { ListDiscussionResponse } from '../../types/list-discussion-response';
import { call } from '@/lib/api/ratel/call';
import { useState } from 'react';
import { useStartMeetingMutation } from '../../hooks/use-start-meeting-mutation';
import { useParticipateMeetingMutation } from '../../hooks/use-participate-meeting-mutation';
import { NavigateFunction, useNavigate } from 'react-router';
import { route } from '@/route';

export class SpaceDiscussionViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public discussion: ListDiscussionResponse,
    public bookmark: State<string | null | undefined>,
    public discussions: State<SpaceDiscussionResponse[]>,
    public t: TFunction<'SpaceDiscussionEditor', undefined>,
    public startMeeting: ReturnType<typeof useStartMeetingMutation>,
    public participantMeeting: ReturnType<typeof useParticipateMeetingMutation>,
    public navigate: NavigateFunction,
  ) {}

  get hasMore() {
    return !!this.bookmark.get();
  }

  enterDiscussionRoom = async (discussionPk: string) => {
    const spacePk = this.spacePk;

    await this.startMeeting.mutateAsync({ spacePk, discussionPk });
    await this.participantMeeting.mutateAsync({ spacePk, discussionPk });

    this.navigate(route.discussionByPk(spacePk, discussionPk));
  };

  loadMore = async () => {
    const bm = this.bookmark.get();
    if (!bm) return;

    const next = await call(
      'GET',
      `/v3/spaces/${encodeURIComponent(this.spacePk)}/discussions?bookmark=${encodeURIComponent(
        bm,
      )}`,
    );

    const page = new ListDiscussionResponse(next);
    const prev = this.discussions.get() ?? [];
    this.discussions.set([...prev, ...page.discussions]);
    this.bookmark.set(page.bookmark ?? null);
  };
}

export function useSpaceDiscussionViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: discussion } = useDiscussionSpace(spacePk);
  const { t } = useTranslation('SpaceDiscussionEditor');

  const startMeeting = useStartMeetingMutation();
  const participantMeeting = useParticipateMeetingMutation();
  const navigate = useNavigate();

  const discussionsState = new State(
    useState<SpaceDiscussionResponse[]>(discussion.discussions || []),
  );
  const bookmarkState = new State(
    useState<string | null>(discussion.bookmark ?? null),
  );

  return new SpaceDiscussionViewerController(
    spacePk,
    space,
    discussion,
    bookmarkState,
    discussionsState,
    t,
    startMeeting,
    participantMeeting,
    navigate,
  );
}
