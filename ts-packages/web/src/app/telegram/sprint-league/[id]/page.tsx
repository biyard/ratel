'use client';

import Loading from '@/app/loading';
import Game, {
  Status as GameStatus,
} from '@/app/spaces/[id]/sprint-league/game';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { SpaceType } from '@/lib/api/models/spaces';
import { ratelApi, useSpaceById } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { route } from '@/route';
import { useParams, useRouter } from 'next/navigation';
import React, { useEffect } from 'react';

export default function Page() {
  const params = useParams();
  const { post } = useApiCall();

  const { data: space, isLoading } = useSpaceById(Number(params.id));
  const isLogin = useLoggedIn();

  const router = useRouter();
  useEffect(() => {
    if (isLoading) {
      return;
    }
    console.log('Space data:', space);
    if (!space || space.space_type !== SpaceType.SprintLeague) {
      router.replace(route.home());
    }
  }, [space, isLogin, router, isLoading]);

  if (isLoading || !space) {
    return <Loading />;
  }
  const sprintLeague = space.sprint_leagues?.[0];
  const players = sprintLeague?.players ?? [];

  const handleVote = async (playerId: number) => {
    const res = await post(
      ratelApi.sprint_league.voteSprintLeague(
        Number(params.id),
        sprintLeague?.id || 0,
      ),
      {
        vote: {
          player_id: playerId,
        },
      },
    );
    if (res.error) {
      throw new Error('Failed to vote', res.error);
    }
  };
  return (
    <Game
      players={players}
      votes={sprintLeague?.votes ?? 0}
      initStatus={
        sprintLeague?.is_voted ? GameStatus.AFTER_VOTE : GameStatus.BEFORE_VOTE
      }
      onVote={handleVote}
      onRepost={() => {
        alert('Repost functionality not implemented yet');
      }}
    />
  );
}
