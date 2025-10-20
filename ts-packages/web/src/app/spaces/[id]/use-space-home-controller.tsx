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
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { logger } from '@/lib/logger';
import { useSpaceUpdateContentMutation } from '@/features/spaces/hooks/use-space-update-content-mutation';
import { showErrorToast } from '@/lib/toast';
import { useSpaceUpdateTitleMutation } from '@/features/spaces/hooks/use-space-update-title-mutation';
import { sideMenusForSpaceType } from '@/features/spaces/utils/side-menus-for-space-type';

export class SpaceHomeController {
  public space: Space;
  public user: UserResponse | null;

  constructor(
    public data: ReturnType<typeof useSpaceHomeData>,
    public state: State<boolean>,
    public t: TFunction<'Space'>,
    public updateSpaceContent: ReturnType<typeof useSpaceUpdateContentMutation>,
    public updateSpaceTitle: ReturnType<typeof useSpaceUpdateTitleMutation>,
  ) {
    this.space = this.data.space.data;
  }

  get timelineItems() {
    // FIXME: add more timeline items even specific to space type
    return [
      {
        label: this.t('timeline_created_at_label'),
        time: this.space.createdAt,
      },
    ];
  }

  get menus() {
    let menus: SideMenuProps[] = [
      {
        Icon: Settings,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_overview'),
      },
    ];

    sideMenusForSpaceType[this.space.spaceType]?.forEach((menu) => {
      let visible = !menu.visible;

      if (typeof menu.visible === 'function') {
        visible = menu.visible(this.space);
      }

      if (visible) {
        menus.push({
          Icon: menu.Icon,
          to: typeof menu.to === 'function' ? menu.to(this.space) : menu.to,
          label: this.t(menu.label),
        });
      }
    });

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

  get isAdmin() {
    return this.space.isAdmin();
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

  handleChange = async (text: string) => {
    try {
      await this.updateSpaceContent.mutateAsync({
        spacePk: this.space.pk,
        content: text,
      });
    } catch (error) {
      logger.error('Failed to update space content', error);
      showErrorToast(`Failed to update space content: ${error}`);
    }
  };

  handleTitleChange = async (title: string) => {
    logger.debug('Title change requested:', title);
    try {
      await this.updateSpaceTitle.mutateAsync({
        spacePk: this.space.pk,
        title,
      });
    } catch (error) {
      logger.error('Failed to update space title', error);
      showErrorToast(`Failed to update space title: ${error}`);
    }
  };

  handleShare = async () => {
    logger.error('handleShare not implemented');
  };

  handleActionEdit = async () => {
    logger.debug('Action edit triggered');
  };

  handleActionSave = async () => {
    logger.debug('Action save triggered');
  };

  handleActionPublish = async () => {
    logger.debug('Action publish triggered');
  };

  get actions() {
    return [
      {
        label: this.t('edit'),
        onClick: this.handleActionEdit,
        holdingLabel: this.t('save'),
        onClickWhileHolding: this.handleActionSave,
      },
      {
        label: this.t('publish'),
        onClick: this.handleActionPublish,
      },
    ];
  }
}

export function useSpaceHomeController(spacePk: string) {
  const data = useSpaceHomeData(spacePk);
  const state = useState(false);
  const { t } = useTranslation('Space');
  const updateSpaceContent = useSpaceUpdateContentMutation();
  const updateSpaceTitle = useSpaceUpdateTitleMutation();

  return new SpaceHomeController(
    data,
    new State(state),
    t,
    updateSpaceContent,
    updateSpaceTitle,
  );
}
