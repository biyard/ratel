import { PieChart1, User, Vote } from '@/components/icons';
import { config } from '@/config';
import { route } from '@/route';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { SpaceType } from '../types/space-type';
import { Trophy } from '@/assets/icons/game';

addSideMenusForSpaceType(SpaceType.Poll, [
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
    Icon: Trophy,
    to: (space) => route.spaceReward(space.pk),
    visible: (space) => !space.isDraft || space.isAdmin(),
    label: 'menu_rewards',
  },
]);
