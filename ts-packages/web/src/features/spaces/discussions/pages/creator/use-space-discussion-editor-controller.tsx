import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import { Space } from '@/features/spaces/types/space';
import { ListDiscussionResponse } from '../../types/list-discussion-response';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import useDiscussionSpace from '../../hooks/use-discussion-space';

export class SpaceDiscussionEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public discussion: ListDiscussionResponse,
    public bookmark: string | null | undefined,
    public discussions: State<SpaceDiscussionResponse[]>,
    public editing: State<boolean>,
  ) {}

  handleEdit = () => {
    this.editing.set(true);
  };

  handleSave = async () => {
    this.editing.set(false);
  };

  handleDiscard = () => {
    this.editing.set(false);
  };
}

export function useSpaceDiscussionEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: discussion } = useDiscussionSpace(spacePk);
  const bookmark = discussion.bookmark;
  const discussions = useState(discussion.discussions || []);
  const editing = useState(false);

  return new SpaceDiscussionEditorController(
    spacePk,
    space,
    discussion,
    bookmark,
    new State(discussions),
    new State(editing),
  );
}
