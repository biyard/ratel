import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import SprintLeagueEditor from '@/features/spaces/sprint-leagues/components/editer';
import SprintLeagueGame from '@/features/spaces/sprint-leagues/components/game';
import { useSprintLeagueController } from './controller';

export default function SpaceSprintLeaguePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }
  const ctrl = useSprintLeagueController(spacePk);

  return (
    <div className="flex flex-col gap-8">
      {space.isAdmin() && space.isDraft && (
        <>
          <SprintLeagueEditor {...ctrl} />
        </>
      )}
      <SprintLeagueGame {...ctrl} />
    </div>
  );
}
