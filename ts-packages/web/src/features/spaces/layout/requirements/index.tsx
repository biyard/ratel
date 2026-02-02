import { SpaceLayoutController } from '../use-space-layout-controller';
import { useRequirmentController } from './use-requirements-controller';

export function Requirements({
  layoutCtrl,
}: {
  layoutCtrl: SpaceLayoutController;
}) {
  const ctrl = useRequirmentController(layoutCtrl);

  return <>{ctrl.component}</>;
}
