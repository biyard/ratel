import { PieChart1, Settings, Vote } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
// import { SpaceStatus } from '../types/space-common';

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
    Icon: PieChart1,
    to: (space) => {
      const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
      return route.spaceAnalyzePollById(space.pk, pollPk);
    },
    visible: (space) => !space.isDraft && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
