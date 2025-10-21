import useSpaceById from '@/hooks/use-space-by-id';
import { Space } from '@/lib/api/models/spaces';
import useDiscussionSpace from '../../hooks/use-discussion-space';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';

export class SpaceDiscussionViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public bookmark: string | null | undefined,
    public discussions: SpaceDiscussionResponse[],
  ) {}
}

export function useSpaceDiscussionViewerController(spacePk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: discussion } = useDiscussionSpace(spacePk);
  const bookmark = discussion.bookmark;
  const discussions = discussion.discussions;

  return new SpaceDiscussionViewerController(
    spacePk,
    space,
    bookmark,
    discussions,
  );
}
