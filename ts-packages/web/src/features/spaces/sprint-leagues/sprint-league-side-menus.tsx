import { Settings, Palace } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
import { SpaceStatus } from '../types/space-common';

addSideMenusForSpaceType(SpaceType.SprintLeague, [
  {
    Icon: Palace,
    to: (space) => route.spaceSprintLeagues(space.pk),
    visible: (space) =>
      space.status !== SpaceStatus.Finished && space.isAdmin(),
    label: 'menu_sprint_league',
  },
  {
    Icon: Settings,
    to: (space) => route.spaceSetting(space.pk),
    visible: (space) =>
      space.status === SpaceStatus.Finished && space.isAdmin(),
    label: 'menu_admin_settings',
  },
]);
