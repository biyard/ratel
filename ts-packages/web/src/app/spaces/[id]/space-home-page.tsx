import { useParams } from 'react-router';
import { useSpaceHomeController } from './use-space-home-controller';

export function SpaceHomePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const _ctrl = useSpaceHomeController(spacePk);

  return (
    <>
      <div>SpaceHomePage</div>
    </>
  );
}
