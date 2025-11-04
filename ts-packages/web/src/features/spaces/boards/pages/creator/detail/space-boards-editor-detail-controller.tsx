import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceBoardsEditorDetailController {
  constructor(
    public spacePk: string,
    public postPk: string,
    public space: Space,
  ) {}
}

export function useSpaceBoardsEditorDetailController(
  spacePk: string,
  postPk: string,
) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceBoardsEditorDetailController(spacePk, postPk, space);
}
