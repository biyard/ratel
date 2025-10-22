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

export class SpaceDiscussionViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public discussion: ListDiscussionResponse,
    public bookmark: State<string | null | undefined>,
    public discussions: State<SpaceDiscussionResponse[]>,
    public t: TFunction<'SpaceDiscussionEditor', undefined>,
  ) {}

  get hasMore() {
    return !!this.bookmark.get();
  }

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
  );
}
