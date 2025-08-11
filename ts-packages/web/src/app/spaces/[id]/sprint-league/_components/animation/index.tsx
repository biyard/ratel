'use client';

import dynamic from 'next/dynamic';

const Base = dynamic(() => import('./base'), {
  ssr: false,
});

import Background, { Dim } from './background';
import { Banner, PlayerNameOverlay, VoteBanner } from './banner';
import Character from './character';
import { useEffect, useRef, useState } from 'react';
import { RepostButton, StartButton, VoteBackButton } from './button';
import VotePlayer from './vote';
import { logger } from '@/lib/logger';
import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';
import { useApplication } from '@pixi/react';
import { showInfoToast } from '@/lib/toast';

const backgroundBundle = [
  {
    alias: 'rank-banner',
    src: '/images/sprint_league/sabana_sheet.png',
  },
  {
    alias: 'banner',
    src: '/images/sprint_league/banner.png',
  },
  {
    alias: 'background',
    src: '/images/sprint_league/background.json',
  },
];
const voteBundle = [
  {
    alias: 'vote-default',
    src: '/images/sprint_league/vote_default.png',
  },
  {
    alias: 'vote-selected',
    src: '/images/sprint_league/vote_selected.png',
  },
  {
    alias: 'vote-unselected',
    src: '/images/sprint_league/vote_unselected.png',
  },
  {
    alias: 'vote-header',
    src: '/images/sprint_league/vote_header.png',
  },
  {
    alias: 'button-vote',
    src: '/images/sprint_league/button_vote.png',
  },
  {
    alias: 'button-repost',
    src: '/images/sprint_league/button_repost.png',
  },
  {
    alias: 'button-back',
    src: '/images/sprint_league/button_back.png',
  },
  {
    alias: 'button-start',
    src: '/images/sprint_league/button_start.png',
  },
];

export interface Player {
  id: number;
  name: string;
  description: string;
  vote_ratio: number;
}

export enum Status {
  BEFORE_VOTE,
  VOTING,
  AFTER_VOTE,
  GAME_END,
}

const SPEED = 1.4;

const BASE_POSITION = [
  { x: 0, y: 430, scale: 1.5 },
  { x: 0, y: 520, scale: 1.5 },
  { x: 0, y: 610, scale: 1.5 },
];

interface SprintLeagueProps extends InnerProps {
  ref: React.RefObject<HTMLDivElement | null>;
}
export default function SprintLeague({
  ref,
  players,
  initStatus,
  votes,
  disabled = false,
  onVote,
  onRepost,
}: SprintLeagueProps) {
  const playerBundle = players
    .map((player) => {
      if (!player.player_images) {
        logger.warn(`Player ${player.id} has no images`);
        return [];
      }
      const alias = player.player_images.alias;
      return [
        {
          alias: `${alias}_run`,
          src: player.player_images.run.json,
        },
        {
          alias: `${alias}_selected`,
          src: player.player_images.select.json,
        },
        {
          alias: `${alias}_win`,
          src: player.player_images.win,
        },
        {
          alias: `${alias}_lose`,
          src: player.player_images.lose,
        },
      ];
    })
    .flat();
  return (
    <Base
      ref={ref}
      bundles={[
        { name: 'background', assets: backgroundBundle },
        { name: 'vote', assets: voteBundle },
        { name: 'characters', assets: playerBundle },
      ]}
    >
      <Inner
        players={players}
        initStatus={initStatus}
        votes={votes}
        onVote={onVote}
        onRepost={onRepost}
        disabled={disabled}
      />
    </Base>
  );
}

export interface InnerProps {
  players: SprintLeaguePlayer[];
  initStatus?: Status;
  votes: number;
  onVote: (playerId: number) => Promise<void>;
  onRepost: () => Promise<void>;
  disabled?: boolean;
}
function Inner({
  players,
  initStatus = Status.BEFORE_VOTE,
  onVote,
  onRepost,
  disabled = false,
}: InnerProps) {
  const { app } = useApplication();
  const [baseSpeed, setBaseSpeed] = useState(
    initStatus !== Status.BEFORE_VOTE && initStatus !== Status.GAME_END
      ? SPEED
      : 0,
  );
  const [status, setStatus] = useState(initStatus);
  const [selectedPlayerId, setSelectedPlayerId] = useState<number | null>(null);
  const [voted, setVoted] = useState(initStatus === Status.AFTER_VOTE);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const timeout = timeoutRef.current;
    return () => {
      if (timeout) {
        clearTimeout(timeout);
      }
    };
  }, []);

  const handleStartVote = () => {
    if (disabled) {
      showInfoToast('Voting is only allowed when the space is published.');
      return;
    }
    setStatus(Status.VOTING);
  };

  const handleVote = async (playerId: number) => {
    try {
      await onVote(playerId);
      setBaseSpeed(SPEED);
      setStatus(Status.AFTER_VOTE);
      setVoted(true);
      timeoutRef.current = setTimeout(() => {
        setVoted(false);
        timeoutRef.current = null;
      }, 5000);
    } catch (error) {
      logger.error('Vote failed:', error);
    }
  };

  const handleBack = () => {
    setStatus(Status.BEFORE_VOTE);
  };

  const handleRepost = async () => {
    try {
      await onRepost();
    } catch (error) {
      logger.error('Repost failed:', error);
    }
  };

  const handleSelectPlayer = (id: number) => {
    setSelectedPlayerId((prev) => (prev === id ? null : id));
  };

  const width = app.screen.width || 0;
  const height = app.screen.height || 0;

  const adjustedWidth = width > height ? (height * 360) / 640 : width;
  const scale = adjustedWidth / 360;
  const totalVotes = players.reduce((acc, player) => acc + player.votes, 0);
  const sortedPlayers = players.sort((a, b) => {
    const aVotes = a.votes || 0;
    const bVotes = b.votes || 0;
    return bVotes - aVotes; // Sort by votes in descending order
  });
  const voteRatios = sortedPlayers.map((player) => {
    const votes = player.votes || 0;
    return totalVotes > 0 ? (votes / totalVotes) * 100 : 0;
  });
  return (
    <>
      <Background alias="background" baseSpeed={baseSpeed} scale={scale} />
      {status <= Status.AFTER_VOTE && (
        <RepostButton onClick={handleRepost} y={630} x={350} scale={scale} />
      )}
      {sortedPlayers.map((player, index) => (
        <Character
          key={player.id}
          index={index}
          scale={scale}
          alias={player.player_images.alias}
          selected={voted && selectedPlayerId === player.id}
          isFinished={status === Status.GAME_END}
          x={
            BASE_POSITION[index].x +
            (status >= Status.AFTER_VOTE ? 2 : 0) * voteRatios[index]
          }
          y={BASE_POSITION[index].y}
          characterScale={BASE_POSITION[index].scale}
          speed={baseSpeed * (0.05 + (voteRatios[index] / 100) * 0.4)}
        />
      ))}
      {status === Status.BEFORE_VOTE && (
        <>
          <Banner scale={scale} />
          <StartButton onClick={handleStartVote} y={100} scale={scale} />
        </>
      )}
      {status === Status.VOTING && (
        <>
          <Dim />
          {selectedPlayerId === null && <VoteBanner scale={scale} />}
          <VotePlayer
            players={players}
            selectedPlayerId={selectedPlayerId}
            onSelect={handleSelectPlayer}
            scale={scale}
          />
          <VoteBackButton
            scale={scale}
            onVote={() => {
              if (selectedPlayerId !== null) {
                handleVote(selectedPlayerId);
              }
            }}
            onBack={handleBack}
          />
        </>
      )}
      {status >= Status.AFTER_VOTE && (
        <PlayerNameOverlay
          names={sortedPlayers.map((player) => player.name)}
          scale={scale}
        />
      )}
    </>
  );
}
