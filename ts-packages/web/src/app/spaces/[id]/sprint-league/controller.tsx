import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import SprintLeague from '@/features/spaces/sprint-leagues/types/sprint-league';
import { SprintLeagueGameProps } from '@/features/spaces/sprint-leagues/components/game';
import { SprintLeagueEditorProps } from '@/features/spaces/sprint-leagues/components/editer';
import useSprintLeague from '@/features/spaces/sprint-leagues/hooks/use-sprint-league';
import SprintLeaguePlayer from '@/features/spaces/sprint-leagues/types/sprint-league-player';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';

export interface ISpaceSprintLeagueController
  extends SprintLeagueGameProps,
    SprintLeagueEditorProps {
  space: Space;
  sprintLeague: SprintLeague;
  isEditMode: boolean;
  t: TFunction<'SprintLeague', undefined>;
  editing: boolean;
  handleEdit: () => void;
  handleSave: () => void;
  handleDiscard: () => void;
}

export function useSprintLeagueController(
  spacePk: string,
): ISpaceSprintLeagueController {
  const { data: space } = useSpaceById(spacePk);
  const { data: sprintLeague } = useSprintLeague(spacePk);
  const { t } = useTranslation('SprintLeague');
  const [editing, setEditing] = useState<boolean>(false);
  const [players, setPlayers] = useState<SprintLeaguePlayer[]>(
    sprintLeague ? sprintLeague.players : [],
  );

  const onVote = async (playerSk: string) => {
    console.log('Vote for playerSk:', playerSk);
    // TODO : implement vote logic
  };

  const onUpdatePlayer = (index, player: SprintLeaguePlayer) => {
    setPlayers((prev) => {
      const newPlayers = [...prev];
      newPlayers[index] = player;
      return newPlayers;
    });
  };

  const handleEdit = () => {
    setEditing(true);
  };

  const handleSave = () => {
    setEditing(false);
  };

  const handleDiscard = () => {
    setEditing(false);
    // Reset players to original sprintLeague players
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
    handleEdit,
    handleSave,
    handleDiscard,
  };
}
