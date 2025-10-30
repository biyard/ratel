import { PieChart1, Vote } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
// import { SpaceStatus } from '../types/space-common';

addSideMenusForSpaceType(SpaceType.Poll, [
  {
    Icon: Vote,
    to: (space) => {
      return route.spacePolls(space.pk);
    },
    label: 'menu_poll',
  },

  {
    Icon: PieChart1,
    to: (space) => {
      return route.spaceAnalyzePolls(space.pk);
    },
    visible: (space) => !space.isDraft && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
