import { TFunction } from 'i18next';
import SprintLeaguePlayer, {
  PlayerImage,
  SpriteSheet,
} from '../../types/sprint-league-player';
import { usePopup } from '@/lib/contexts/popup-service';
import Card from '@/components/card';
import { Textarea } from '@/components/ui/textarea';
import { Button } from '@/components/ui/button';
import unknown from '@/assets/images/unknown.png';
import { useState } from 'react';
import { Input } from '@/components/ui/input';
import PlayerSelectModal from '../select-player-modal';

export interface SprintLeagueEditorProps {
  players?: SprintLeaguePlayer[];
  t: TFunction<'SprintLeague', undefined>;
  onUpdatePlayer: (index: number, player: SprintLeaguePlayer) => void;
}
interface SprintLeagueEditorState {
  sk: string;
  playerImage?: PlayerImage;
  name?: string;
  description?: string;
}

export default function SprintLeagueEditor({
  t,
  players,
  onUpdatePlayer,
}: SprintLeagueEditorProps) {
  const popup = usePopup();

  const [playersState, setPlayersState] = useState<SprintLeagueEditorState[]>(
    players?.map((p) => ({
      sk: p.sk,
      playerImage: p.player_image || null,
      name: p.name || '',
      description: p.description || '',
    })) || [],
  );

  const updatePlayer = (index: number, player: SprintLeagueEditorState) => {
    setPlayersState((prev) => {
      const newState = [...prev];
      newState[index] = player;
      return newState;
    });
  };
  return (
    <div>
      <Card>
        <div className="w-full flex flex-col gap-8">
          <div className="font-bold text-white text-[15px]/[20px]">
            {t('sprint_players')}
          </div>
          <div className="flex flex-col gap-10 w-full">
            {players.map((player, index) => (
              <div key={player.sk} className="flex flex-row min-h-50 gap-5">
                <div className="aspect-square">
                  <PlayerSelector
                    handleSelect={() => {
                      popup
                        .open(
                          <PlayerSelectModal
                            onSelect={(playerImage: PlayerImage) => {
                              updatePlayer(index, {
                                ...playersState[index],
                                playerImage,
                              });
                              popup.close();
                            }}
                          />,
                        )
                        .withTitle(t('select_player_modal_title'));
                    }}
                    t={t}
                  />
                </div>
                <div className="flex flex-col flex-1 gap-2.5">
                  <Input
                    className="w-full text-[15px]/[23px] px-4 py-6 text-foreground"
                    value={player.name}
                    onChange={(e) => {
                      updatePlayer(index, {
                        ...player,
                        name: e.currentTarget.value || '',
                      });
                    }}
                  />
                  <Textarea
                    className="flex-1 text-foreground"
                    value={player.description || ''}
                    onChange={(e) =>
                      updatePlayer(index, {
                        ...player,
                        description: e.currentTarget.value || '',
                      })
                    }
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>
    </div>
  );
}
function PlayerSelector({
  t,
  spriteSheet,
  handleSelect,
}: {
  t: TFunction<'SprintLeague', undefined>;
  spriteSheet?: SpriteSheet;
  handleSelect: () => void;
}) {
  return (
    <div className="relative bg-neutral-800 light:bg-transparent border light:border-neutral-300 rounded-lg">
      <Button
        variant="default"
        size="sm"
        className="z-1 absolute -translate-x-1/2 -translate-y-3/4 top-3/4 left-1/2"
        onClick={handleSelect}
      >
        {t('select_character')}
      </Button>
      <div className="aspect-square size-75 rounded-lg overflow-hidden">
        {spriteSheet ? (
          // <Character spriteSheet={spriteSheet} />
          <div> Character Here </div>
        ) : (
          <img src={unknown} alt="Unknown Player" />
        )}
      </div>
    </div>
  );
}
