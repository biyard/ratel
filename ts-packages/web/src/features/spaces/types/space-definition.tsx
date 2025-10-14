import { SpaceType } from './space-type';
import { Discuss, Palace, Mega, Vote } from '@/components/icons';
import { Cube } from '@/assets/icons/shopping';

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
    type: SpaceType.Notice,
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
    experiment: true,
    canBoost: true,
  },
  {
    type: SpaceType.dAgit,
    Icon: <Cube className="[&>path]:stroke-[var(--color-neutral-500)]" />,
    labelKey: 'dAgit.label',
    descKey: 'dAgit.desc',
    experiment: true,
    canBoost: false,
  },
];
