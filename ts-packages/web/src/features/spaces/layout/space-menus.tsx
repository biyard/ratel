import {
  Discuss,
  PieChart1,
  Post,
  GameTrophy,
  User,
  Vote,
} from '@/components/icons';
import { config } from '@/config';
import { route } from '@/route';
import { SpaceType } from '../types/space-type';
import { SideMenu } from './types/space-menu';

const pieChartColored = (props) => (
  <PieChart1
    {...props}
    className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500 w-5 h-5"
  />
);

export const SPACE_MENUS: Record<SpaceType, SideMenu[]> = {
  [SpaceType.Poll]: [
    {
      Icon: Vote,
      to: (space) => {
        const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
        return route.spacePollById(space.pk, pollPk);
      },
      label: 'menu_poll',
    },
    {
      Icon: User,
      to: (space) => route.spaceMembers(space.pk),
      visible: () => config.experiment,
      label: 'menu_members',
    },
    {
      Icon: PieChart1,
      to: (space) => {
        const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
        return route.spaceAnalyzePollById(space.pk, pollPk);
      },
      visible: (space) => !space.isDraft && space.isAdmin(),
      label: 'menu_analyze',
    },
    {
      Icon: GameTrophy,
      to: (space) => route.spaceReward(space.pk),
      visible: (space) =>
        (config.experiment && !space.isDraft) || space.isAdmin(),
      label: 'menu_rewards',
    },
  ],

  [SpaceType.Deliberation]: [
    {
      Icon: Post,
      to: (space) => route.spaceFiles(space.pk),
      label: 'menu_files',
    },
    {
      Icon: Vote,
      to: (space) => route.spacePolls(space.pk),
      label: 'menu_poll',
    },
    {
      Icon: Post,
      to: (space) => route.spaceBoards(space.pk),
      label: 'menu_boards',
    },
    {
      Icon: Discuss,
      to: (space) => route.spaceDao(space.pk),
      visible: (space) =>
        config.experiment &&
        space.authorType === 2 &&
        (space.isAdmin() || Boolean(space.daoAddress)),
      label: 'menu_dao',
    },
    {
      Icon: User,
      to: (space) => route.spaceMembers(space.pk),
      visible: (space) => space.isAdmin(),
      label: 'menu_members',
    },
    {
      Icon: User,
      to: (space) => route.spacePanels(space.pk),
      visible: (space) => space.isAdmin(),
      label: 'menu_panels',
    },
    {
      Icon: pieChartColored,
      to: (space) => route.spaceAnalyzePolls(space.pk),
      visible: (space) => !space.isDraft && space.isAdmin(),
      label: 'menu_analyze',
    },
  ],

  [SpaceType.Nft]: [],
  [SpaceType.SprintLeague]: [],
  [SpaceType.Legislation]: [],
  [SpaceType.Commitee]: [],
  [SpaceType.Quiz]: [],
  [SpaceType.dAgit]: [],
};
