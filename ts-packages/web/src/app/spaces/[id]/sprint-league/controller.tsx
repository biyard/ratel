import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import SprintLeague from '@/features/spaces/sprint-leagues/types/sprint-league';
import { SprintLeagueGameProps } from '@/features/spaces/sprint-leagues/components/game';
import { SprintLeagueEditorProps } from '@/features/spaces/sprint-leagues/components/editer';
import useSprintLeague from '@/features/spaces/sprint-leagues/hooks/use-sprint-league';
import SprintLeaguePlayer, {
  defaultPlayer,
} from '@/features/spaces/sprint-leagues/types/sprint-league-player';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import CreateSprintLeaguePlayer from '@/features/spaces/sprint-leagues/types/create-sprint-league-player';
import { useUpdateSprintLeagueMutation } from '@/features/spaces/sprint-leagues/hooks/use-update-sprint-league-mutation';

export interface ISpaceSprintLeagueController
  extends SprintLeagueGameProps,
    SprintLeagueEditorProps {
  space: Space;
  sprintLeague: SprintLeague;
  isEditMode: boolean;
  t: TFunction<'SpaceSprintLeague', undefined>;
  editing: boolean;
}

export function useSprintLeagueController(
  spacePk: string,
): ISpaceSprintLeagueController {
  const { data: space } = useSpaceById(spacePk);
  const { data: sprintLeague } = useSprintLeague(spacePk);
  const { t } = useTranslation('SpaceSprintLeague');
  const [editing, setEditing] = useState<boolean>(false);
  const [players, setPlayers] = useState<SprintLeaguePlayer[]>(
    sprintLeague.players && sprintLeague.players.length !== 0
      ? sprintLeague.players
      : [defaultPlayer(), defaultPlayer(), defaultPlayer()],
  );
  const updateSprintLeague = useUpdateSprintLeagueMutation().mutateAsync;
  const onVote = async (playerSk: string) => {
    console.log('Vote for playerSk:', playerSk);
    // TODO : implement vote logic
  };

  const onUpdatePlayer = (index: number, player: SprintLeaguePlayer) => {
    setPlayers((prev) => {
      const newPlayers = [...prev];
      newPlayers[index] = player;
      return newPlayers;
    });
  };

  const onEdit = () => {
    setEditing(true);
  };

  const onSave = async () => {
    console.log('Players', players);
    const createReq = players.map((player) => {
      const req: CreateSprintLeaguePlayer = {
        name: player.name,
        description: player.description,
        player_image: player.player_image,
      };
      return req;
    });
    await updateSprintLeague({
      spacePk: spacePk,
      players: createReq,
    });
    setEditing(false);
  };

  const onDiscard = () => {
    setEditing(false);
    if (sprintLeague) {
      setPlayers(sprintLeague.players);
    }
  };
  return {
    space: space,
    sprintLeague: sprintLeague,
    isEditMode: space?.isAdmin() && space?.isDraft ? true : false,
    t,
    players,
    editing,
    onVote,
    onUpdatePlayer,

    initialStatus: 0,

    disabled: false,
    onEdit,
    onSave,
    onDiscard,
  };
}
