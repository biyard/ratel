import useDiscussionSpace from '../../hooks/use-discussion-space';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { ListDiscussionResponse } from '../../types/list-discussion-response';

export class SpaceDiscussionViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public discussion: ListDiscussionResponse,
    public bookmark: string | null | undefined,
    public discussions: SpaceDiscussionResponse[],
    public t: TFunction<'SpaceDiscussionEditor', undefined>,
  ) {}
}

export function useSpaceDiscussionViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: discussion } = useDiscussionSpace(spacePk);
  const bookmark = discussion.bookmark;
  const discussions = discussion.discussions;
  const { t } = useTranslation('SpaceDiscussionEditor');

  return new SpaceDiscussionViewerController(
    spacePk,
    space,
    discussion,
    bookmark,
    discussions,
    t,
  );
}
