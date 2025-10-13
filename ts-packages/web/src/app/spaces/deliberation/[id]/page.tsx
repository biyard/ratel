import { useParams } from 'react-router';
import { SpaceHeaderProvider } from '../../_components/header/provider';
import SpaceHeader from '../../_components/header';
import { useDeliberationSpaceController } from './use-deliberation-space-controller';

export default function DeliberationSpacePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useDeliberationSpaceController(spacePk);
  const hasEditPermission = true;

  return (
    <div className="flex flex-col w-full gap-6">
      <SpaceHeaderProvider
        post={ctrl.post.post}
        space={ctrl.space}
        hasEditPermission={hasEditPermission}
        onSave={ctrl.onSave}
      >
        <SpaceHeader />
      </SpaceHeaderProvider>
    </div>
  );
}
