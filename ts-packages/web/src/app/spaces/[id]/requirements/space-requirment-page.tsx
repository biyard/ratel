import { useRequirmentController } from '@/features/spaces/requirments/controller';

import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useParams } from 'react-router';

export default function SpaceRequirementPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);
  if (!space) {
    throw new Error('Space not found');
  }

  const ctrl = useRequirmentController(spacePk);

  return ctrl.getComponent();
}
