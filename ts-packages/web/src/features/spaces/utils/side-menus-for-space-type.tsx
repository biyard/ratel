import { logger } from '@/lib/logger';
import { SpaceType } from '../types/space-type';
import { Space } from '../types/space';

export const sideMenusForSpaceType = {};

export type SideMenu = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string | ((args: Space) => string);
  label: string;
  visible?: (args: Space) => boolean;
};

export function addSideMenusForSpaceType(
  spaceType: SpaceType,
  menus: Array<SideMenu>,
) {
  if (!sideMenusForSpaceType[spaceType]) {
    sideMenusForSpaceType[spaceType] = menus;
  } else {
    logger.error('Menus for this space type already exist:', spaceType);
  }
}
