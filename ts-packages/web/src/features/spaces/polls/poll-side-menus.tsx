import { Settings } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
import { SpaceStatus } from '../types/space-common';

addSideMenusForSpaceType(SpaceType.Poll, [
  {
    Icon: Settings,
    to: (space) => {
      const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
      return route.spacePollById(space.pk, pollPk);
    },
    label: 'menu_poll',
  },

  {
    Icon: Settings,
    to: (space) => route.spacePolls(space.pk),
    visible: (space) =>
      space.status === SpaceStatus.Finished && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
