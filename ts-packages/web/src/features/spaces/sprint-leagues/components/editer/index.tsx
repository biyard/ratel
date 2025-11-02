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
import { Input } from '@/components/ui/input';
import PlayerSelectModal from '../select-player-modal';
import { Row } from '@/components/ui/row';
import Character from '../character';

export interface SprintLeagueEditorProps {
  players?: SprintLeaguePlayer[];
  t: TFunction<'SpaceSprintLeague', undefined>;
  editing: boolean;
  onUpdatePlayer: (index: number, player: SprintLeaguePlayer) => void;
  onEdit: () => void;
  onSave: () => Promise<void>;
  onDiscard: () => void;
}

export default function SprintLeagueEditor({
  t,
  players,
  editing,
  onUpdatePlayer,
  onEdit,
  onSave,
  onDiscard,
}: SprintLeagueEditorProps) {
  const popup = usePopup();

  const updatePlayer = (
    index: number,
    req: {
      name?: string;
      description?: string;
      playerImage?: PlayerImage;
    },
  ) => {
    const player = players[index];
    if (req.name !== undefined) {
      player.name = req.name;
    }
    if (req.description !== undefined) {
      player.description = req.description;
    }
    if (req.playerImage !== undefined) {
      player.player_image = req.playerImage;
    }

    onUpdatePlayer(index, player);
  };

  return (
    <>
      <Row className="gap-2 justify-end mb-4">
        {editing ? (
          <>
            <Button variant="primary" onClick={onSave}>
              {t('btn_save')}
            </Button>
            <Button onClick={onDiscard}>{t('btn_discard')}</Button>
          </>
        ) : (
          <Button onClick={onEdit}>{t('btn_edit')}</Button>
        )}
      </Row>
      {editing && (
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
                                  playerImage,
                                });
                                popup.close();
                              }}
                              t={t}
                            />,
                          )
                          .withTitle(t('select_player_modal_title'));
                      }}
                      spriteSheet={
                        player.player_image.run.image
                          ? player.player_image.run
                          : undefined
                      }
                      t={t}
                    />
                  </div>
                  <div className="flex flex-col flex-1 gap-2.5">
                    <Input
                      className="w-full text-[15px]/[23px] px-4 py-6 text-foreground"
                      value={player.name}
                      onChange={(e) => {
                        updatePlayer(index, {
                          name: e.currentTarget.value || '',
                        });
                      }}
                    />
                    <Textarea
                      className="flex-1 text-foreground"
                      value={player.description}
                      onChange={(e) =>
                        updatePlayer(index, {
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
      )}
    </>
  );
}
function PlayerSelector({
  t,
  spriteSheet,
  handleSelect,
}: {
  t: TFunction<'SpaceSprintLeague', undefined>;
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
        {t('btn_select_character')}
      </Button>
      <div className="aspect-square size-75 rounded-lg overflow-hidden">
        {spriteSheet ? (
          <Character spriteSheet={spriteSheet} />
        ) : (
          <img src={unknown} alt="Unknown Player" />
        )}
      </div>
    </div>
  );
}
