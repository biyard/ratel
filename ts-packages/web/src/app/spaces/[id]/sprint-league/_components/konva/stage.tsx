'use client';

import { Group, Layer, Stage, Text } from 'react-konva';
import { useEffect, useState } from 'react';
import { CHARACTER_SIZE, Status, VIEWPORT_HEIGHT, VIEWPORT_WIDTH } from '.';
import { SprintLeaguePlayer } from '@/lib/api/models/sprint_league';
import Background from './components/background';
import DimOverlay from './components/dim-overlay';
import CharacterSprite from './components/character';
import StoppedImage from './components/image';
import { Banner, BannerVote } from './components/banner';
import { BackButton, StartButton, VoteButton } from './components/button';
import VoteItem from './components/vote';
import PlayerNameOverlay from './components/name-overlay';
import { showErrorToast } from '@/lib/toast';

const BASE_POSITION = [
  { x: CHARACTER_SIZE / 4 + 10, y: 300, scale: 1 },
  { x: CHARACTER_SIZE / 4 + 10, y: 400, scale: 1 },
  { x: CHARACTER_SIZE / 4 + 10, y: 500, scale: 1 },
];

const STOP_POSITION = [
  { x: 90, y: 0 },
  { x: 50, y: 0 },
  { x: 10, y: 0 },
];
const FINISH_POSITION = [
  { x: VIEWPORT_WIDTH / 2 - 10, y: 300, scale: 1 },
  { x: VIEWPORT_WIDTH / 2 - 100, y: 450, scale: 0.7 },
  { x: VIEWPORT_WIDTH / 2 - 100, y: 550, scale: 0.7 },
];

export default function KonvaCanvas({
  initialStatus = Status.BEFORE_START,
  players,
  onVote,
  width,
  height,
}: {
  initialStatus?: Status;
  players: SprintLeaguePlayer[];
  onVote: (playerId: number) => Promise<void>;
  disabled: boolean;
  width: number;
  height: number;
}) {
  const [status, setStatus] = useState<Status>(initialStatus);
  const [selectedPlayerId, setSelectedPlayerId] = useState<number | null>(null);

  useEffect(() => {
    if (selectedPlayerId !== null) {
      const timer = setTimeout(() => {
        setSelectedPlayerId(null);
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [selectedPlayerId]);
  return (
    <Stage
      width={width}
      height={height}
      scale={{ x: width / VIEWPORT_WIDTH, y: height / VIEWPORT_HEIGHT }}
    >
      <Background baseSpeed={status === Status.AFTER_VOTE ? 150 : 0} />
      <CharacterRace
        players={players}
        isRaceStarted={status > Status.VOTE}
        isFinished={status === Status.GAME_END}
        selectedPlayerId={selectedPlayerId}
      />
      {status === Status.BEFORE_START && (
        <BeforeStart handleStart={() => setStatus(Status.VOTE)} />
      )}
      <DimOverlay visible={status === Status.VOTE} />

      {status === Status.VOTE && (
        <Vote
          players={players}
          handleVote={async (playerId: number) => {
            try {
              await onVote(playerId);
              setStatus(Status.AFTER_VOTE);
              setSelectedPlayerId(playerId);
            } catch (error) {
              console.error('Vote failed:', error);
              showErrorToast('Vote failed. Please try again.');
              return;
            }
          }}
          handleBack={() => setStatus(Status.BEFORE_START)}
        />
      )}
      {status >= Status.AFTER_VOTE && (
        <AfterVote
          names={players.sort((a, b) => b.votes - a.votes).map((p) => p.name)}
        />
      )}
    </Stage>
  );
}
function CharacterRace({
  players,
  isRaceStarted,
  isFinished,
  selectedPlayerId,
}: {
  players: SprintLeaguePlayer[];
  isRaceStarted: boolean;
  isFinished?: boolean;
  selectedPlayerId: number | null;
}) {
  const totalVotes = players.reduce((sum, p) => sum + p.votes, 0);
  const minVotes = Math.min(...players.map((p) => p.votes));
  console.log('totalVotes', totalVotes);
  const minSpeed = isRaceStarted ? 0.5 : 0;

  const voteRatios = isRaceStarted
    ? players.map((p) =>
        totalVotes > 0 ? (p.votes - minVotes) / totalVotes : 1 / players.length,
      )
    : [0, 0, 0];

  return (
    <Layer>
      {players
        .sort((a, b) => b.votes - a.votes)
        .map((player, index) =>
          !isFinished ? (
            <CharacterSprite
              key={player.id}
              imageUrl={
                selectedPlayerId !== player.id
                  ? player.player_images.run.image
                  : player.player_images.select.image
              }
              jsonUrl={
                selectedPlayerId !== player.id
                  ? player.player_images.run.json
                  : player.player_images.select.json
              }
              x={
                BASE_POSITION[index].x +
                (!isRaceStarted ? STOP_POSITION[index].x : 0) +
                140 * voteRatios[index]
              } // 0 ~ 180
              y={BASE_POSITION[index].y - 50 * voteRatios[index]} // 0 ~ 50
              scale={BASE_POSITION[index].scale + voteRatios[index] / 2}
              speed={
                Math.max(voteRatios[index] * 5, minSpeed) +
                (selectedPlayerId === player.id ? 5 : 0)
              }
            />
          ) : (
            <StoppedImage
              key={player.id}
              imageUrl={
                index === 0
                  ? player.player_images.win
                  : player.player_images.lose
              }
              x={FINISH_POSITION[index].x}
              y={FINISH_POSITION[index].y}
              scale={FINISH_POSITION[index].scale}
            />
          ),
        )}
    </Layer>
  );
}
function BeforeStart({ handleStart }: { handleStart: () => void }) {
  return (
    <Layer>
      <Banner y={10} />
      <StartButton x={0} y={110} onClick={handleStart} />
    </Layer>
  );
}

function Vote({
  players,
  handleVote,
  handleBack,
}: {
  players: SprintLeaguePlayer[];
  handleVote: (playerId: number) => void;
  handleBack: () => void;
}) {
  const [selectedPlayerId, setSelectedPlayerId] = useState<number | null>(null);

  const handleSelect = (id: number) => {
    setSelectedPlayerId((prev) => (prev === id ? null : id));
  };

  return (
    <Layer>
      <BannerVote y={0} />
      <Group y={120}>
        {selectedPlayerId !== null && (
          <Text
            x={0}
            y={0}
            text={players.find((p) => p.id === selectedPlayerId)?.name || ''}
            fontSize={25}
            fontStyle="900"
            align="center"
            fill="#ffffff"
            width={VIEWPORT_WIDTH}
            listening={false}
          />
        )}

        <Group width={VIEWPORT_WIDTH} y={160}>
          {players.map((p, index) => (
            <VoteItem
              key={p.id}
              x={index * 110 + 70}
              y={0}
              isSelected={
                selectedPlayerId === null ? null : selectedPlayerId === p.id
              }
              jsonUrl={p.player_images.run.json}
              imageUrl={p.player_images.run.image}
              onClick={() => handleSelect(p.id)}
            />
          ))}
        </Group>

        <Group y={300}>
          <Text
            width={VIEWPORT_WIDTH}
            y={0}
            text={
              selectedPlayerId === null
                ? 'SELECT YOUR PLAYER'
                : players.find((p) => p.id === selectedPlayerId)?.description ||
                  ''
            }
            fontSize={16}
            fontStyle="bold"
            fill="#ffffff"
            wrap="word"
            align="center"
            listening={false}
          />
        </Group>
      </Group>
      <Group y={VIEWPORT_HEIGHT - 100}>
        <BackButton x={-5} onClick={handleBack} disabled={false} />
        <VoteButton
          x={165}
          disabled={selectedPlayerId === null}
          onClick={() => {
            if (selectedPlayerId !== null) {
              handleVote(selectedPlayerId);
            }
          }}
        />
      </Group>
    </Layer>
  );
}

function AfterVote({ names }: { names: string[] }) {
  return (
    <Layer>
      <PlayerNameOverlay names={names.slice(0, 3)} />
    </Layer>
  );
}
