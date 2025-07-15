'use client';

import dynamic from 'next/dynamic';

const Base = dynamic(() => import('./base'), {
  ssr: false,
});

import Background, { Dim } from './background';
import { Banner, PlayerNameOverlay, VoteBanner } from './banner';
import Character from './character';
import { useState } from 'react';
import { RepostButton, StartButton, VoteBackButton } from './button';
import VotePlayer from './vote';
import { logger } from '@/lib/logger';
import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';
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
}

const SPEED = 1.4;

// const RANK_POSITION = [
//   { x: 80, y: 430, scale: 3, speed: 0.3 },
//   { x: 50, y: 520, scale: 2, speed: 0.2 },
//   { x: 0, y: 600, scale: 1.5, speed: 0.1 },
// ];

const POSITION = [
  { x: 0, y: 430, scale: 1.5 },
  { x: 0, y: 520, scale: 1.5 },
  { x: 0, y: 610, scale: 1.5 },
];

export default function SprintLeagueGame({
  players,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars, unused-imports/no-unused-vars
  votes = 0,
  initStatus = Status.BEFORE_VOTE,
  onVote,
  onRepost,
}: {
  initStatus: Status;
  votes: number;
  players: SprintLeaguePlayer[];
  onVote: (playerId: number) => Promise<void>;
  onRepost: () => void;
}) {
  const [baseSpeed, setBaseSpeed] = useState(
    initStatus === Status.BEFORE_VOTE ? 0 : SPEED,
  );
  const [status, setStatus] = useState(initStatus);
  const [selectedPlayerId, setSelectedPlayerId] = useState<number | null>(null);
  const [voted, setVoted] = useState(initStatus === Status.AFTER_VOTE);
  const handleStartVote = () => {
    setStatus(Status.VOTING);
  };

  const player_bundles = players
    .map((player) => {
      return [
        {
          alias: `player-${player.id}-run`,
          src: player.player_images.run.json,
        },
        {
          alias: `player-${player.id}-select`,
          src: player.player_images.select.json,
        },
        {
          alias: `player-${player.id}-win`,
          src: player.player_images.win,
        },
        {
          alias: `player-${player.id}-lose`,
          src: player.player_images.lose,
        },
      ];
    })
    .flat();

  const handleVote = async (playerId: number) => {
    try {
      await onVote(playerId);
      setBaseSpeed(SPEED);
      setStatus(Status.AFTER_VOTE);
      setVoted(true);
      setTimeout(() => {
        setVoted(false);
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

  return (
    <Base
      bundles={[
        {
          name: 'background',
          assets: backgroundBundle,
        },
        {
          name: 'vote',
          assets: voteBundle,
        },
        {
          name: 'characters',
          assets: player_bundles,
        },
      ]}
    >
      <Background alias="background" baseSpeed={baseSpeed} />
      {players.map((player, index) => (
        <Character
          key={player.id}
          playerId={player.id}
          selected={voted && selectedPlayerId === player.id}
          x={POSITION[index].x}
          y={POSITION[index].y}
          scale={POSITION[index].scale}
          speed={baseSpeed * 0.3}
        />
      ))}
      {status === Status.BEFORE_VOTE && (
        <>
          <Banner />
          <StartButton onClick={handleStartVote} y={100} />
          <RepostButton onClick={handleRepost} y={630} x={350} />
        </>
      )}
      {status === Status.VOTING && (
        <>
          <Dim />
          {selectedPlayerId === null && <VoteBanner />}
          <VotePlayer
            players={players}
            selectedPlayerId={selectedPlayerId}
            onSelect={handleSelectPlayer}
          />
          <VoteBackButton
            onVote={() => {
              if (selectedPlayerId !== null) {
                handleVote(selectedPlayerId);
              }
            }}
            onBack={handleBack}
          />
        </>
      )}
      {status === Status.AFTER_VOTE && (
        <PlayerNameOverlay names={players.map((player) => player.name)} />
      )}
    </Base>
  );
}
