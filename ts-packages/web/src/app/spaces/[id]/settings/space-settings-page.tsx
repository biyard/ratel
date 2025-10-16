import { useParams } from 'react-router';
import { useSpaceSettingsController } from './use-space-settings-controller';

export function SpaceSettingsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();

  const _ctrl = useSpaceSettingsController(spacePk);

  return (
    <>
      <div>SpaceSettingsPage</div>
    </>
  );
}
