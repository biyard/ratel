import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceInvitationEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceInvitationEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  return new SpaceInvitationEditorController(spacePk, space);
}
