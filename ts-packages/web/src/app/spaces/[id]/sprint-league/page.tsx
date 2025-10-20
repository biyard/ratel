import { useParams } from 'react-router';
import '@/features/spaces/sprint-leagues/sprint-league-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import SprintLeagueEditor from '@/features/spaces/sprint-leagues/components/editer';
import SprintLeagueGame from '@/features/spaces/sprint-leagues/components/game';
import { useSprintLeagueController } from './controller';
import { Row } from '@/components/ui/row';
import { Button } from '@/components/ui/button';

export default function SpaceSprintLeaguePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }
  const ctrl = useSprintLeagueController(spacePk);

  return (
    <div className="flex flex-row gap-8">
      <div className="flex-1">
        <SprintLeagueGame {...ctrl} />
      </div>
      {space.isAdmin() && space.isDraft && (
        <div className="flex-1">
          <Row className="gap-2 justify-end mb-4">
            {ctrl.editing ? (
              <>
                <Button variant="primary" onClick={ctrl.handleSave}>
                  {ctrl.t('btn_save')}
                </Button>
                <Button onClick={ctrl.handleDiscard}>
                  {ctrl.t('btn_discard')}
                </Button>
              </>
            ) : (
              <Button onClick={ctrl.handleEdit}>{ctrl.t('btn_edit')}</Button>
            )}
          </Row>
          <SprintLeagueEditor {...ctrl} />
        </div>
      )}
    </div>
  );
}
