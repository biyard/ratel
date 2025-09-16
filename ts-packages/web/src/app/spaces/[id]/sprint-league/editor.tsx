'use client';

import { useCallback, useEffect } from 'react';
import {
  useEditCoordinatorStore,
  type CommonEditableData,
} from '../space-store';
import { useSprintLeagueStore } from './sprint-league-store';
import useSpaceById, { useUpdateSpace } from '@/hooks/use-space-by-id';
import { useSprintLeagueSpaceByIdMutation } from '@/hooks/use-sprint-league-by-id';
import { SpaceStatus, spaceUpdateRequest } from '@/lib/api/models/spaces';
import SpaceContents from '../_components/space-contents';
import PlayerEdit from './_components/player';

import SprintLeagueGame, { Status as GameStatus } from './_components/konva';
import { showErrorToast } from '@/lib/toast';

export function SprintLeagueEditor({ spaceId }: { spaceId: number }) {
  const {
    isEdit,
    setPageSaveHandler,
    updateCommonData,
    setSpacePublishValidator,
  } = useEditCoordinatorStore();
  const { initialize } = useSprintLeagueStore();

  const { data: space } = useSpaceById(spaceId);
  const sprintLeague = space?.sprint_leagues?.[0];
  const isDraft = space.status === SpaceStatus.Draft;
  const { mutateAsync: updateMutateAsync } = useUpdateSpace(spaceId);

  const {
    votePlayer: { mutateAsync: votePlayerMutateAsync },
    updatePlayer: { mutateAsync: updatePlayerMutateAsync },
  } = useSprintLeagueSpaceByIdMutation(spaceId);

  const storedPlayers = useSprintLeagueStore((state) => state.players);

  const publishValidator = useCallback(() => {
    const players = Object.values(useSprintLeagueStore.getState().players);
    if (players.length === 0) {
      showErrorToast('At least one player is required to publish.');
      return false;
    }
    for (const player of players) {
      if (!player.name?.trim() || !player.description?.trim()) {
        showErrorToast('All players must have a name and description.');
        return false;
      }
      if (!player.player_images.run.image?.trim()) {
        showErrorToast('All players must have an Character');
        return false;
      }
    }
    return true;
  }, []);
  const saveHandler = useCallback(
    async (commonData: Partial<CommonEditableData>) => {
      if (!space) {
        return false;
      }

      const playersToSave = Object.values(
        useSprintLeagueStore.getState().players,
      );

      const sprintLeagueId = space.sprint_leagues?.[0]?.id;
      if (playersToSave.length > 0 && !sprintLeagueId) {
        console.warn('Sprint League ID is missing; cannot save players.');
        return false;
      }

      try {
        await Promise.all([
          ...playersToSave.map((player) =>
            updatePlayerMutateAsync({
              playerId: player.id,
              sprintLeagueId: sprintLeagueId as number,
              req: {
                name: player.name,
                description: player.description,
                player_images: player.player_images,
              },
            }),
          ),
          updateMutateAsync(
            spaceUpdateRequest(
              commonData.html_contents ?? '',
              [],
              [],
              [],
              [],
              [],
              commonData.title,
              commonData.started_at,
              commonData.ended_at,
            ),
          ),
        ]);
        return true;
      } catch (error) {
        console.error('Save failed:', error);
        return false;
      }
    },
    [space, updateMutateAsync, updatePlayerMutateAsync],
  );

  useEffect(() => {
    if (!sprintLeague?.players || sprintLeague.players.length === 0) {
      initialize([]);
      return;
    }

    const storedPlayers = useSprintLeagueStore.getState().players;
    if (Object.keys(storedPlayers).length > 0) {
      return;
    }

    initialize(sprintLeague.players);
  }, [sprintLeague?.players, initialize, spaceId]);

  useEffect(() => {
    if (isEdit) {
      setPageSaveHandler(saveHandler);
    }
    setSpacePublishValidator(publishValidator);
  }, [
    isEdit,
    setPageSaveHandler,
    saveHandler,
    setSpacePublishValidator,
    publishValidator,
  ]);

  const handleVote = async (playerId: number) => {
    await votePlayerMutateAsync({
      playerId,
      sprintLeagueId: space.sprint_leagues?.[0].id ?? 0,
    });
  };

  return (
    <>
      <SpaceContents
        isEdit={isEdit}
        htmlContents={space?.html_contents ?? ''}
        setContents={(newContents) =>
          updateCommonData({ html_contents: newContents })
        }
      />

      <div className="w-full h-full flex justify-center top-0 left-0 bg-black max-mobile:absolute max-mobile:overflow-hidden max-mobile:h-screen max-mobile:w-screen">
        <div className="min-w-[360px] max-w-[1080px] h-auto aspect-[36/64]">
          <SprintLeagueGame
            initialStatus={
              space.status === SpaceStatus.Finish
                ? GameStatus.GAME_END
                : sprintLeague?.is_voted
                  ? GameStatus.AFTER_VOTE
                  : GameStatus.BEFORE_START
            }
            players={Object.values(storedPlayers)}
            onVote={handleVote}
            disabled={isDraft}
          />
        </div>
      </div>
      {isDraft && <PlayerEdit isEdit={isEdit} />}
    </>
  );
}
