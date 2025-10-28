import {
  CheckCircle2,
  Discuss,
  PieChart1,
  Post,
  Vote,
} from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';
// import { SpaceStatus } from '../types/space-common';

const checkCircle2Colored = (props) => (
  <CheckCircle2
    {...props}
    className="[&>path]:stroke-neutral-80 [&>circle]:stroke-neutral-80"
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
      const pollPk = `SPACE_POLL#${space.pk.split('#')[1]}`;
      return route.spacePollById(space.pk, pollPk);
    },
    label: 'menu_poll',
  },
  {
    Icon: Discuss,
    to: (space) => {
      return route.spaceDiscussions(space.pk);
    },
    label: 'menu_discussions',
  },
  {
    Icon: checkCircle2Colored,
    to: (space) => {
      return route.spaceRecommendations(space.pk);
    },
    label: 'menu_recommendations',
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
