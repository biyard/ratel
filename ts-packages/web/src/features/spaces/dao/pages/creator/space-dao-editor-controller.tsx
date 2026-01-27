import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceDaoEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceDaoEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceDaoEditorController(spacePk, space);
}
