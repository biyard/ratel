'use client';

import Loading from '@/app/loading';
import SprintLeagueGame, {
  Status as GameStatus,
} from '@/app/spaces/[id]/sprint-league/_components/animation';

import useSpaceById, { useShareSpace } from '@/hooks/use-space-by-id';
import { useSprintLeagueSpaceByIdMutation } from '@/hooks/use-sprint-league-by-id';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { SpaceStatus, SpaceType } from '@/lib/api/models/spaces';
import { route } from '@/route';
import { useParams, useRouter } from 'next/navigation';
import React, { useEffect } from 'react';

export default function Page() {
  const params = useParams();
  const ref = React.useRef<HTMLDivElement>(null);
  const { data: space, isLoading } = useSpaceById(Number(params.id));
  const isLogin = useLoggedIn();

  const router = useRouter();
  useEffect(() => {
    if (isLoading) {
      return;
    }
    if (!space || space.space_type !== SpaceType.SprintLeague) {
      router.replace(route.home());
    }
  }, [space, isLogin, router, isLoading]);

  const {
    votePlayer: { mutateAsync: votePlayerMutateAsync },
  } = useSprintLeagueSpaceByIdMutation(space.id);

  const { mutateAsync: shareSpaceMutateAsync } = useShareSpace(space.id);
  if (isLoading || !space) {
    return <Loading />;
  }
  const sprintLeague = space.sprint_leagues?.[0];
  const players = sprintLeague?.players ?? [];

  const handleVote = async (playerId: number) => {
    await votePlayerMutateAsync({
      playerId,
      sprintLeagueId: space.sprint_leagues?.[0].id ?? 0,
    });
  };

  const handleShare = async () => {
    await shareSpaceMutateAsync();
  };

  return (
    <div
      ref={ref}
      className="flex flex-col justify-center items-center w-full h-full"
    >
      <SprintLeagueGame
        ref={ref}
        players={players}
        votes={sprintLeague?.votes ?? 0}
        initStatus={
          space.status === SpaceStatus.Finish
            ? GameStatus.GAME_END
            : sprintLeague?.is_voted
              ? GameStatus.AFTER_VOTE
              : GameStatus.BEFORE_VOTE
        }
        onVote={handleVote}
        onRepost={handleShare}
      />
    </div>
  );
}
