import {
  Discuss,
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

const pieChartColored = (props) => (
  <PieChart1
    {...props}
    className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500 w-5 h-5"
  />
);

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
  {
    Icon: Post,
    to: (space) => {
      return route.spaceBoards(space.pk);
    },
    // visible: (space) => space.participated || space.isAdmin(),
    label: 'menu_boards',
  },
  {
    Icon: Discuss,
    to: (space) => {
      return route.spaceDao(space.pk);
    },
    visible: (space) => config.experiment && space.isAdmin(),
    label: 'menu_dao',
  },
  {
    Icon: Discuss,
    to: (space) => {
      return route.spaceIncentive(space.pk);
    },
    visible: (space) =>
      config.experiment &&
      (space.isAdmin() || Boolean(space.daoAddress)) &&
      space.isFinished,
    label: 'menu_incentive',
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
    visible: (space) => space.isAdmin(),
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

  // {
  //   Icon: checkCircle2Colored,
  //   to: (space) => {
  //     return route.spaceRecommendations(space.pk);
  //   },
  //   label: 'menu_recommendations',
  // },
  {
    Icon: pieChartColored,
    to: (space) => {
      return route.spaceAnalyzePolls(space.pk);
    },
    visible: (space) => !space.isDraft && space.isAdmin(),
    label: 'menu_analyze',
  },
]);
