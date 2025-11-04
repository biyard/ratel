import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceBoardsCreateController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceBoardsCreateController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);

  return new SpaceBoardsCreateController(spacePk, space);
}
