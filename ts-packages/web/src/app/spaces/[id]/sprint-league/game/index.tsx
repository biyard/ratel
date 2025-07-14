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

const userBundle0 = [
  {
    alias: 'user_0_run',
    src: '/images/sprint_league/lee_jun_run.json',
  },
  {
    alias: 'user_0_select',
    src: '/images/sprint_league/lee_jun_selected.json',
  },
  {
    alias: 'user_0_win',
    src: '/images/sprint_league/lee_jun_win.png',
  },
  {
    alias: 'user_0_lose',
    src: '/images/sprint_league/lee_jun_lose.png',
  },
];

const userBundle1 = [
  {
    alias: 'user_1_run',
    src: '/images/sprint_league/kim_moon_run.json',
  },
  {
    alias: 'user_1_select',
    src: '/images/sprint_league/kim_moon_selected.json',
  },
  {
    alias: 'user_1_win',
    src: '/images/sprint_league/kim_moon_win.png',
  },
  {
    alias: 'user_1_lose',
    src: '/images/sprint_league/kim_moon_lose.png',
  },
];

const userBundle2 = [
  {
    alias: 'user_2_run',
    src: '/images/sprint_league/lee_jae_run.json',
  },
  {
    alias: 'user_2_select',
    src: '/images/sprint_league/lee_jae_selected.json',
  },
  {
    alias: 'user_2_win',
    src: '/images/sprint_league/lee_jae_win.png',
  },
  {
    alias: 'user_2_lose',
    src: '/images/sprint_league/lee_jae_lose.png',
  },
];

interface Player {
  id: number;
  name: string;
  description: string;
  vote_ratio: number;
}
export default function SprintLeagueGame({
  players,
  initStatus = Status.BEFORE_VOTE,
  onVote,
  onRepost,
}: {
  initStatus: Status;
  players: Player[];
  onVote: (playerId: number) => Promise<void>;
  onRepost: () => Promise<void>;
}) {
  const [baseSpeed, setBaseSpeed] = useState(
    initStatus === Status.BEFORE_VOTE ? 0 : 1.4,
  );
  const [status, setStatus] = useState(initStatus);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [voted, setVoted] = useState(false);
  const handleStartVote = () => {
    setStatus(Status.VOTING);
  };
  const players_sorted = players.sort((a, b) => a.vote_ratio - b.vote_ratio);

  const handleVote = async (selectedPlayer: number) => {
    try {
      await onVote(players_sorted[selectedPlayer].id);
      setBaseSpeed(1.4);
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

  const handleSelectPlayer = (index: number) => {
    setSelectedIndex((prev) => (prev === index ? null : index));
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
          assets: [...userBundle0, ...userBundle1, ...userBundle2],
        },
      ]}
    >
      <Background alias="background" baseSpeed={baseSpeed} />
      <Character
        selected={voted && selectedIndex === 0}
        index={0}
        x={80}
        y={430}
        speed={baseSpeed * 0.3}
        scale={3}
      />
      <Character
        selected={voted && selectedIndex === 1}
        index={1}
        x={50}
        y={520}
        speed={baseSpeed * 0.2}
        scale={2}
      />
      <Character
        selected={voted && selectedIndex === 2}
        index={2}
        x={0}
        y={600}
        speed={baseSpeed * 0.1}
        scale={1.5}
      />
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
          {selectedIndex === null && <VoteBanner />}
          <VotePlayer
            players={players}
            selectedIndex={selectedIndex}
            onSelect={handleSelectPlayer}
          />
          <VoteBackButton
            onVote={() => {
              if (selectedIndex !== null) {
                handleVote(selectedIndex);
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

export enum Status {
  BEFORE_VOTE,
  VOTING,
  AFTER_VOTE,
}
