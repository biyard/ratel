import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceBoardsEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceBoardsEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceBoardsEditorController(spacePk, space);
}
