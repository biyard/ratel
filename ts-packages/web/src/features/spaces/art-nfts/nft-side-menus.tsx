import { Shares, ShoppingCube } from '@/components/icons';
import { SpaceType } from '../types/space-type';
import { addSideMenusForSpaceType } from '../utils/side-menus-for-space-type';
import { route } from '@/route';

addSideMenusForSpaceType(SpaceType.Nft, [
  {
    Icon: ShoppingCube,
    to: (space) => {
      return route.spaceNftPreview(space.pk);
    },
    label: 'menu_nft_preview',
  },

  {
    Icon: Shares,
    to: (space) => {
      return route.spaceNftArtTwin(space.pk);
    },
    label: 'menu_nft_art_twin',
    visible: (space) => space.isAdmin() || !space.isDraft,
  },

  // {
  //   Icon: Settings,
  //   to: (space) => {
  //     return route.createArtwork(spacePkToPostPk(space.pk));
  //   },
  //   label: 'menu_nft_settings',
  //   visible: (space) => space.isAdmin(),
  // },
]);
