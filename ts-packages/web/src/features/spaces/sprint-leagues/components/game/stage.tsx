import { Group, Layer, Stage, Text } from 'react-konva';
import { useEffect, useState } from 'react';
import { CHARACTER_SIZE, VIEWPORT_HEIGHT, VIEWPORT_WIDTH } from './constants';
import Background from './background';
import DimOverlay from './dim-overlay';
import CharacterSprite from './character';
import StoppedImage from './image';
import { Banner, BannerVote } from './banner';
import { BackButton, StartButton, VoteButton } from './button';
import VoteItem from './vote';
import PlayerNameOverlay from './name-overlay';
import { showErrorToast } from '@/lib/toast';
import SprintLeaguePlayer from '../../types/sprint-league-player';
import { Status } from '.';

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
  disabled,
  width,
  height,
}: {
  initialStatus?: Status;
  players: SprintLeaguePlayer[];
  onVote: (playerSk: string) => Promise<void>;
  disabled: boolean;
  width: number;
  height: number;
}) {
  const [status, setStatus] = useState<Status>(
    disabled ? Status.BEFORE_START : initialStatus,
  );
  const [selectedPlayerSk, setselectedPlayerSk] = useState<string | null>(null);

  useEffect(() => {
    if (selectedPlayerSk !== null) {
      const timer = setTimeout(() => {
        setselectedPlayerSk(null);
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [selectedPlayerSk]);
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
        selectedPlayerSk={selectedPlayerSk}
      />
      {status === Status.BEFORE_START && (
        <BeforeStart
          handleStart={() => {
            if (disabled) return;
            setStatus(Status.VOTE);
          }}
        />
      )}
      <DimOverlay visible={status === Status.VOTE} />

      {status === Status.VOTE && (
        <Vote
          players={players}
          handleVote={async (playerSk: string) => {
            try {
              await onVote(playerSk);
              setStatus(Status.AFTER_VOTE);
              setselectedPlayerSk(playerSk);
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
  selectedPlayerSk,
}: {
  players: SprintLeaguePlayer[];
  isRaceStarted: boolean;
  isFinished?: boolean;
  selectedPlayerSk: string | null;
}) {
  const totalVotes = players.reduce((sum, p) => sum + p.votes, 0);
  const minVotes = Math.min(...players.map((p) => p.votes));
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
              key={player.pk}
              imageUrl={
                selectedPlayerSk !== player.sk
                  ? player.player_image.run.image
                  : player.player_image.select.image
              }
              jsonUrl={
                selectedPlayerSk !== player.sk
                  ? player.player_image.run.json
                  : player.player_image.select.json
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
                (selectedPlayerSk === player.sk ? 5 : 0)
              }
            />
          ) : (
            <StoppedImage
              key={player.sk}
              imageUrl={
                index === 0 ? player.player_image.win : player.player_image.lose
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
  handleVote: (playerSk: string) => void;
  handleBack: () => void;
}) {
  const [selectedPlayerSk, setSelectedPlayerSk] = useState<string | null>(null);

  const handleSelect = (sk: string) => {
    setSelectedPlayerSk((prev) => (prev === sk ? null : sk));
  };

  return (
    <Layer>
      <BannerVote y={0} />
      <Group y={120}>
        {selectedPlayerSk !== null && (
          <Text
            x={0}
            y={0}
            text={players.find((p) => p.sk === selectedPlayerSk)?.name || ''}
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
              key={p.sk}
              x={index * 110 + 70}
              y={0}
              isSelected={
                selectedPlayerSk === null ? null : selectedPlayerSk === p.sk
              }
              jsonUrl={p.player_image.run.json}
              imageUrl={p.player_image.run.image}
              onClick={() => handleSelect(p.sk)}
            />
          ))}
        </Group>

        <Group y={300}>
          <Text
            width={VIEWPORT_WIDTH}
            y={0}
            text={
              selectedPlayerSk === null
                ? 'SELECT YOUR PLAYER'
                : players.find((p) => p.sk === selectedPlayerSk)?.description ||
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
          disabled={selectedPlayerSk === null}
          onClick={() => {
            if (selectedPlayerSk !== null) {
              handleVote(selectedPlayerSk);
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
