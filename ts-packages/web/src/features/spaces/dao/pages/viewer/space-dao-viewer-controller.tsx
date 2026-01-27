import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceDaoViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceDaoViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceDaoViewerController(spacePk, space);
}
