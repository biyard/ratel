import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceBoardsViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceBoardsViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceBoardsViewerController(spacePk, space);
}
