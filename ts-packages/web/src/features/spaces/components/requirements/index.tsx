import { useController } from './use-controller';

export function Requirements() {
  const ctrl = useController();

  return <>{ctrl.component}</>;
}
