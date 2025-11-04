import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceBoardsViewerDetailController {
  constructor(
    public spacePk: string,
    public postPk: string,
    public space: Space,
  ) {}
}

export function useSpaceBoardsViewerDetailController(
  spacePk: string,
  postPk: string,
) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceBoardsViewerDetailController(spacePk, postPk, space);
}
