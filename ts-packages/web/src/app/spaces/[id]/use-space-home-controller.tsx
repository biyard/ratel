import { useState } from 'react';
import { State } from '@/types/state';
import { useSpaceHomeData } from './use-space-home-data';
import { SideMenuProps } from '@/features/spaces/components/space-side-menu';
import { route } from '@/route';
import { Space } from '@/features/spaces/types/space';
import { Settings, Vote } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { UserResponse } from '@/lib/api/ratel/me.v3';
import { SpaceType } from '@/features/spaces/types/space-type';
import { SpaceStatus } from '@/features/spaces/types/space-common';

export class SpaceHomeController {
  public space: Space;
  public user: UserResponse | null;

  constructor(
    public data: ReturnType<typeof useSpaceHomeData>,
    public state: State<boolean>,
    public t: TFunction<'Space'>,
  ) {
    this.space = this.data.space.data;
  }

  get menusGenerator() {
    const gen = {};
    gen[SpaceType.Poll] = this.pollMenus;
    gen[SpaceType.Quiz] = this.quizMenus;
    gen[SpaceType.Deliberation] = this.deliberationMenus;

    return gen;
  }

  get menus() {
    let menus: SideMenuProps[] = [
      {
        Icon: Settings,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_overview'),
      },
    ];

    if (this.menusGenerator[this.space.spaceType]) {
      menus = menus.concat(this.menusGenerator[this.space.spaceType]);
    }

    if (this.space.isAdmin()) {
      menus = menus.concat(this.adminMenus);
    }

    return menus;
  }

  get pollMenus(): SideMenuProps[] {
    return [
      {
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_poll'),
      },
    ];
  }

  get quizMenus(): SideMenuProps[] {
    return [
      {
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_quiz'),
      },
    ];
  }

  get deliberationMenus(): SideMenuProps[] {
    const common = [
      {
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_discussions'),
      },

      {
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_poll'),
      },
      {
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_files'),
      },
    ];

    if (this.space.status === SpaceStatus.Finished) {
      common.push({
        Icon: Vote,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_recommendations'),
      });
    }

    return common;
  }

  get adminMenus(): SideMenuProps[] {
    return [
      {
        Icon: Settings,
        to: route.spaceSetting(this.space.pk),
        label: this.t('menu_admin_settings'),
      },
    ];
  }
}

export function useSpaceHomeController(spacePk: string) {
  const data = useSpaceHomeData(spacePk);
  const state = useState(false);
  const { t } = useTranslation('Space');

  return new SpaceHomeController(data, new State(state), t);
}
