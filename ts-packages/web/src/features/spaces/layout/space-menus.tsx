import {
  Discuss,
  PieChart1,
  Post,
  GameTrophy,
  User,
  Vote,
  Settings,
} from '@/components/icons';
import { config } from '@/config';
import { route } from '@/route';
import { SpaceType } from '../types/space-type';
import { Space } from '../types/space';
import { UserType } from '@/lib/api/ratel/users.v3';

export type SideMenu = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: (space: Space) => string;
  label: string;
  visible?: (space: Space) => boolean;
};

export enum Label {
  Overview = 'menu_overview',
  Poll = 'menu_poll',
  Discussions = 'menu_discussions',
  Panels = 'menu_panels',
  Boards = 'menu_boards',
  Members = 'menu_members',
  Files = 'menu_files',
  Dao = 'menu_dao',
  Quiz = 'menu_quiz',
  AdminSettings = 'menu_admin_settings',
  Rewards = 'menu_rewards',
  Analyze = 'menu_analyze',
  Requirements = 'menu_requirements',
}

export const REQUIRE_MENUS: SideMenu[] = [
  {
    Icon: Vote,
    to: (space) => route.spaceRequirements(space.pk),
    label: Label.Requirements,
  },
];
export const BASE_MENUS: SideMenu[] = [
  {
    Icon: Post,
    to: (space) => route.spaceByType(space.spaceType, space.pk),
    label: Label.Overview,
  },
];
export const ADMIN_MENUS: SideMenu[] = [
  {
    Icon: Settings,
    to: (space) => route.spaceSetting(space.pk),
    label: Label.AdminSettings,
  },
];
export const SPACE_MENUS: Record<SpaceType, SideMenu[]> = {
  [SpaceType.Poll]: [
    {
      Icon: Vote,
      to: (space) => {
        const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
        return route.spacePollById(space.pk, pollPk);
      },
      label: Label.Poll,
    },
    {
      Icon: User,
      to: (space) => route.spaceMembers(space.pk),
      visible: () => config.experiment,
      label: Label.Members,
    },
    {
      Icon: PieChart1,
      to: (space) => {
        const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
        return route.spaceAnalyzePollById(space.pk, pollPk);
      },
      visible: (space) => !space.isDraft && space.isAdmin(),
      label: Label.Analyze,
    },
    {
      Icon: GameTrophy,
      to: (space) => route.spaceReward(space.pk),
      visible: (space) => !space.isDraft || space.isAdmin(),
      label: Label.Rewards,
    },
  ],

  [SpaceType.Deliberation]: [
    {
      Icon: Post,
      to: (space) => route.spaceFiles(space.pk),
      label: Label.Files,
    },
    {
      Icon: Vote,
      to: (space) => route.spacePolls(space.pk),
      label: Label.Poll,
    },
    {
      Icon: Post,
      to: (space) => route.spaceBoards(space.pk),
      label: Label.Boards,
    },
    {
      Icon: Discuss,
      to: (space) => route.spaceDao(space.pk),
      visible: (space) =>
        config.experiment &&
        space.authorType === UserType.Team &&
        (space.isAdmin() || Boolean(space.daoAddress)),
      label: Label.Dao,
    },
    {
      Icon: User,
      to: (space) => route.spaceMembers(space.pk),
      visible: (space) => space.isAdmin(),
      label: Label.Members,
    },
    {
      Icon: User,
      to: (space) => route.spacePanels(space.pk),
      visible: (space) => space.isAdmin(),
      label: Label.Panels,
    },
    {
      Icon: PieChart1,
      to: (space) => route.spaceAnalyzePolls(space.pk),
      visible: (space) => !space.isDraft && space.isAdmin(),
      label: Label.Analyze,
    },
  ],

  [SpaceType.Nft]: [],
  [SpaceType.SprintLeague]: [],
  [SpaceType.Legislation]: [],
  [SpaceType.Commitee]: [],
  [SpaceType.Quiz]: [],
  [SpaceType.dAgit]: [],
};
