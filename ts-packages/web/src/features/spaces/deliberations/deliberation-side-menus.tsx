import { Settings } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
// import { SpaceStatus } from '../types/space-common';

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
      const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
      return route.spacePollById(space.pk, pollPk);
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
    to: (space) => {
      const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
      return route.spaceAnalyzePollById(space.pk, pollPk);
    },
    visible: (space) => !space.isDraft && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
