import { Settings } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
import { SpaceStatus } from '../types/space-common';

addSideMenusForSpaceType(SpaceType.Deliberation, [
  {
    Icon: Settings,
    to: (space) => {
      return route.spaceFiles(space.pk);
    },
    label: 'menu_files',
  },
  {
    Icon: Settings,
    to: (space) => {
      return route.spacePolls(space.pk);
    },
    label: 'menu_poll',
  },
  {
    Icon: Settings,
    to: (space) => {
      return route.spaceDiscussions(space.pk);
    },
    label: 'menu_discussions',
  },
  {
    Icon: Settings,
    to: (space) => {
      return route.spaceRecommendations(space.pk);
    },
    label: 'menu_recommendations',
  },
  {
    Icon: Settings,
    to: (space) => route.spacePolls(space.pk),
    visible: (space) =>
      space.status === SpaceStatus.Finished && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
