import {
  // CheckCircle2,
  // Discuss,
  PieChart1,
  Post,
  User,
  Vote,
} from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
import { config } from '@/config';
// import { SpaceStatus } from '../types/space-common';

// const checkCircle2Colored = (props) => (
//   <CheckCircle2
//     {...props}
//     className="[&>path]:stroke-neutral-80 [&>circle]:stroke-neutral-80"
//   />
// );

addSideMenusForSpaceType(SpaceType.Deliberation, [
  {
    Icon: Post,
    to: (space) => {
      return route.spaceFiles(space.pk);
    },
    label: 'menu_files',
  },
  {
    Icon: Vote,
    to: (space) => {
      return route.spacePolls(space.pk);
    },
    label: 'menu_poll',
  },
  // {
  //   Icon: Discuss,
  //   to: (space) => {
  //     return route.spaceDiscussions(space.pk);
  //   },
  //   label: 'menu_discussions',
  // },
  {
    Icon: User,
    to: (space) => {
      return route.spaceMembers(space.pk);
    },
    visible: () => config.experiment,
    label: 'menu_members',
  },
  {
    Icon: User,
    to: (space) => {
      return route.spacePanels(space.pk);
    },
    visible: (space) => space.isAdmin(),
    label: 'menu_panels',
  },
  {
    // FIXME: fix this icon
    Icon: User,
    to: (space) => {
      return route.spaceBoards(space.pk);
    },
    visible: () => config.experiment,
    label: 'menu_boards',
  },
  // {
  //   Icon: checkCircle2Colored,
  //   to: (space) => {
  //     return route.spaceRecommendations(space.pk);
  //   },
  //   label: 'menu_recommendations',
  // },
  {
    Icon: PieChart1,
    to: (space) => {
      return route.spaceAnalyzePolls(space.pk);
    },
    visible: (space) => !space.isDraft && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
