import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';

export class SpaceInvitationViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
  ) {}
}

export function useSpaceInvitationViewerController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  return new SpaceInvitationViewerController(spacePk, space);
}
