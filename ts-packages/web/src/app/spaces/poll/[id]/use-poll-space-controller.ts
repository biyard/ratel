import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { PollSpaceResponse } from '@/lib/api/ratel/poll.spaces.v3';
import usePollSpace from '@/features/poll-space/hooks/use-poll-space';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';
import {
  SpaceHeaderController,
  useSpaceHeader,
} from '@/features/spaces/components/header/use-space-header';
import { useState } from 'react';

export class PollSpaceController {
  public post: PostDetailResponse;
  public space: PollSpaceResponse;
  public headerCtrl: SpaceHeaderController;

  isEditMode: boolean;
  constructor(
    post: PostDetailResponse,
    space: PollSpaceResponse,
    headerCtrl: SpaceHeaderController,
  ) {
    this.post = post;
    this.space = space;
    this.headerCtrl = headerCtrl;
    this.isEditMode = headerCtrl.isEditingMode;
  }
}

export function usePollSpaceController(spacePk: string): PollSpaceController {
  const { data: space } = usePollSpace(spacePk);
  const { data: feed } = useFeedById(space.post_pk);

  const onSave = async (title: string, html_content: string) => {
    // Implement save logic here
    console.log('Save changes', { title, html_content });
  };
  const hasEditPermission = true; // TODO: replace with actual permission check

  const headerCtrl = useSpaceHeader(
    feed.post,
    space,
    hasEditPermission,
    onSave,
  );
  return new PollSpaceController(feed, space, headerCtrl);
}
