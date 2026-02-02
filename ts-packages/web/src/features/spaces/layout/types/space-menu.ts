import { Space } from '../../types/space';

/**
 * Side menu item definition for Space layout
 */
export type SideMenu = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string | ((space: Space) => string);
  label: string;
  visible?: (space: Space) => boolean;
};
