import { useParams } from 'react-router';
import { usePollSpaceController } from './use-poll-space-controller';
import { SpaceHeaderProvider } from '../../_components/header/provider';
import SpaceHeader from '../../_components/header';

export default function PollSpacePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = usePollSpaceController(spacePk);
  const hasEditPermission = true; // TODO: replace with actual permission check

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
