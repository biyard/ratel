import { SpaceType } from './space-type';
import { Discuss, Palace, Mega, Vote, ShoppingCube } from '@/components/icons';

export type SpaceDefinition = {
  type: SpaceType;
  Icon: React.JSX.Element;
  labelKey: string;
  descKey: string;
  experiment?: boolean;
  canBoost?: boolean;
};

export const SPACE_DEFINITIONS: SpaceDefinition[] = [
  {
    type: SpaceType.Poll,
    Icon: <Vote />,
    labelKey: 'poll.label',
    descKey: 'poll.desc',
    canBoost: true,
  },
  {
    type: SpaceType.Quiz,
    Icon: <Mega />,
    labelKey: 'notice.label',
    descKey: 'notice.desc',
    canBoost: true,
  },
  {
    type: SpaceType.Deliberation,
    Icon: <Discuss />,
    labelKey: 'deliberation.label',
    descKey: 'deliberation.desc',
    canBoost: false,
  },
  {
    type: SpaceType.SprintLeague,
    Icon: <Palace className="[&>path]:stroke-[var(--color-neutral-500)]" />,
    labelKey: 'sprintLeague.label',
    descKey: 'sprintLeague.desc',
    canBoost: true,
  },

  {
    type: SpaceType.Nft,
    Icon: (
      <ShoppingCube className="[&>path]:stroke-[var(--color-neutral-500)]" />
    ),
    labelKey: 'nft.label',
    descKey: 'nft.desc',
    experiment: true,
    canBoost: false,
  },
];
