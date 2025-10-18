import { logger } from '@/lib/logger';
import { SideMenuProps } from '../components/space-side-menu';
import { SpaceType } from '../types/space-type';
import { Space } from '../types/space';

export const sideMenusForSpaceType = {};

export type SideMenu = SideMenuProps & {
  visible: (args: Space) => boolean;
};

export function addSideMenusForSpaceType(
  spaceType: SpaceType,
  menus: Array<SideMenuProps>,
) {
  if (!sideMenusForSpaceType[spaceType]) {
    sideMenusForSpaceType[spaceType] = menus;
  } else {
    logger.error('Menus for this space type already exist:', spaceType);
  }
}
