import { useState } from 'react';
import { State } from '@/types/state';
import { useSpaceHomeData } from './use-space-home-data';
import { SideMenuProps } from '@/features/spaces/components/space-side-menu';
import { route } from '@/route';
import { Space } from '@/features/spaces/types/space';
import { Post, Settings } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { UserResponse } from '@/lib/api/ratel/me.v3';
import { logger } from '@/lib/logger';
import { useSpaceUpdateContentMutation } from '@/features/spaces/hooks/use-space-update-content-mutation';
import { showErrorToast } from '@/lib/toast';
import { useSpaceUpdateTitleMutation } from '@/features/spaces/hooks/use-space-update-title-mutation';
import { sideMenusForSpaceType } from '@/features/spaces/utils/side-menus-for-space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import PublishSpaceModal from '@/features/spaces/modals/space-publish-modal';
import { usePublishSpaceMutation } from '@/features/spaces/hooks/use-publish-mutation';
import SpaceDeleteModal from '@/features/spaces/modals/space-delete-modal';
import { useDeleteSpaceMutation } from '@/features/spaces/hooks/use-delete-mutation';
import { NavigateFunction, useNavigate } from 'react-router';

export class SpaceHomeController {
  public space: Space;
  public user: UserResponse | null;
  public saveHook?: () => Promise<void>;
  public publishHook?: () => Promise<void>;

  constructor(
    public navigate: NavigateFunction,
    public data: ReturnType<typeof useSpaceHomeData>,
    public state: State<boolean>,
    public t: TFunction<'Space'>,
    public updateSpaceContent: ReturnType<typeof useSpaceUpdateContentMutation>,
    public updateSpaceTitle: ReturnType<typeof useSpaceUpdateTitleMutation>,
    public editState: State<boolean>,
    public saveState: State<boolean>,
    public popup: ReturnType<typeof usePopup>,
    public publishSpace: ReturnType<typeof usePublishSpaceMutation>,
    public deleteSpace: ReturnType<typeof useDeleteSpaceMutation>,
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
    const menus: SideMenuProps[] = [
      {
        Icon: Post,
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

    /* if (this.space.isAdmin()) {
     *   menus = menus.concat(this.adminMenus);
     * } */

    return menus;
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
    this.editState.set(true);
  };

  handleActionSave = async () => {
    logger.debug('Action save triggered');
    this.editState.set(false);
    if (this.saveHook) {
      this.saveHook();
    }
  };

  handlePublish = async (publishType) => {
    logger.debug('Publishing space with type:', publishType);
    if (this.publishHook) {
      this.publishHook();
    }

    const visibility = { type: publishType };

    this.publishSpace.mutateAsync({
      spacePk: this.space.pk,
      visibility,
    });
    this.popup.close();
  };

  handleDelete = async () => {
    if (this.publishHook) {
      this.publishHook();
    }

    this.deleteSpace.mutateAsync({
      spacePk: this.space.pk,
    });
    this.popup.close();
    this.navigate(route.home());
  };

  handleActionPublish = async () => {
    logger.debug('Action publish triggered');

    this.popup
      .open(<PublishSpaceModal onPublish={this.handlePublish} />)
      .withTitle(this.t('publish_space'))
      .withoutBackdropClose();
  };

  handleActionDelete = async () => {
    logger.debug('Action delete triggered');

    this.popup
      .open(
        <SpaceDeleteModal
          spaceName={this.space.title}
          onDelete={this.handleDelete}
          onClose={() => {
            this.popup.close();
          }}
        />,
      )
      .withTitle(this.t('delete_space'))
      .withoutBackdropClose();
  };

  get actions() {
    const ret = [
      {
        label: this.t('delete'),
        onClick: this.handleActionDelete,
      },
    ];

    if (this.space.isDraft) {
      ret.unshift({
        label: this.t('publish'),
        onClick: this.handleActionPublish,
      });
    }

    return ret;
  }
}

export function useSpaceHomeController(spacePk: string) {
  const data = useSpaceHomeData(spacePk);
  const state = useState(false);
  const { t } = useTranslation('Space');
  const navigate = useNavigate();
  const updateSpaceContent = useSpaceUpdateContentMutation();
  const updateSpaceTitle = useSpaceUpdateTitleMutation();
  const publishSpace = usePublishSpaceMutation();
  const deleteSpace = useDeleteSpaceMutation();

  const edit = useState(false);
  const save = useState(false);
  const popup = usePopup();

  return new SpaceHomeController(
    navigate,
    data,
    new State(state),
    t,
    updateSpaceContent,
    updateSpaceTitle,
    new State(edit),
    new State(save),
    popup,
    publishSpace,
    deleteSpace,
  );
}
