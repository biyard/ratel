'use client';

import Game, {
  Status as GameStatus,
} from '@/app/spaces/[id]/sprint-league/game';
import { useLoggedIn } from '@/lib/api/hooks/users';
import { SpaceType } from '@/lib/api/models/spaces';
import { useSpaceById } from '@/lib/api/ratel_api';
import { route } from '@/route';
import { useParams, useRouter } from 'next/navigation';
import React, { useEffect } from 'react';

export default function Page() {
  const params = useParams();

  console.log('Sprint League Game Page', params.id);

  const { data: space, isLoading } = useSpaceById(93);
  const isLogin = useLoggedIn();

  const router = useRouter();
  useEffect(() => {
    if (isLoading) {
      return;
    }
    console.log('Space data:', space);
    if (!space || space.space_type !== SpaceType.SprintLeague || !isLogin) {
      router.replace(route.home());
    }
  }, [space, isLogin, router, isLoading]);

  // 4. 데이터가 로딩 중이거나 없을 경우 로딩 UI를 보여줍니다.
  if (isLoading || !space) {
    return <div>Loading...</div>; // 또는 다른 로딩 스피너 컴포넌트
  }

  const players = [
    {
      id: 1,
      name: '이준석',
      description: '이준석은 빠른 스프린트로 유명합니다.',
      vote_ratio: 0.2,
    },
    {
      id: 2,
      name: '이재명',
      description: '이재명은 전략적인 플레이로 유명합니다.',
      vote_ratio: 0.3,
    },
    {
      id: 3,
      name: '김문수',
      description: '김문수는 강력한 방어로 유명합니다.',
      vote_ratio: 0.5,
    },
  ];

  return (
    <Game
      players={players}
      initStatus={GameStatus.BEFORE_VOTE}
      onVote={function (playerId: number): Promise<void> {
        throw new Error('Function not implemented.');
      }}
      onRepost={function (): Promise<void> {
        throw new Error('Function not implemented.');
      }}
    />
  );
}
