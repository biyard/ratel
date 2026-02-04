import { useMemo, useCallback, useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router';
import { route } from '@/route';
import {
  ADMIN_MENUS,
  BASE_MENUS,
  REQUIRE_MENUS,
  SPACE_MENUS,
} from './space-menus';
import { useSpaceById } from '../hooks/use-space-by-id';
import { useUserInfo } from '@/hooks/use-user-info';
import useFileSpace from '../files/hooks/use-file-space';
import { SpaceType } from '../types/space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import { usePublishSpaceMutation } from '../hooks/use-publish-mutation';
import { useStartSpaceMutation } from '../hooks/use-start-mutation';
// import { useFinishSpaceMutation } from '../hooks/use-finish-mutation';
import { useDeleteSpaceMutation } from '../hooks/use-delete-mutation';
import { useParticipateSpaceMutation } from '../hooks/use-participate-space-mutation';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { Space } from '../types/space';
import { UserDetailResponse } from '@/lib/api/ratel/users.v3';
import { FileResponse } from '../files/types/file-response';
import { SpaceVisibility } from '../types/space-common';
import PublishSpaceModal, { PublishType } from '../modals/space-publish-modal';
import SpaceDeleteModal from '../modals/space-delete-modal';
import SpaceStartModal from '../modals/space-start-modal';
import { SpaceParticipantProfileProps } from './components/space-participant-profile';
import { type I18nSpaceLayout, useSpaceLayoutI18n } from './space-layout-i18n';
import { useSpaceUpdateTitleMutation } from '../hooks/use-space-update-title-mutation';

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
  tag?: {
    label: string;
    visible: boolean;
  };
};

export enum Role {
  Admin,
  Participant,
  Viewer,
}

export interface LayoutAction {
  label: string;
  onClick: () => Promise<void> | void;
}

export interface SpaceLayoutController {
  space: Space;
  user: UserDetailResponse | null;
  profile: SpaceParticipantProfileProps;
  files: FileResponse;
  menus: SideMenuProps[];
  currentMenu: SideMenuProps;
  timelineItems: Array<{ label: string; time: number }>;
  navigate: ReturnType<typeof useNavigate>;
  popup: ReturnType<typeof usePopup>;
  i18n: I18nSpaceLayout;
  adminActions: LayoutAction[];
  viewerActions: LayoutAction[];
  participantActions: LayoutAction[];

  role: Role;
  handleTitleChange: (title: string) => void;
}

export function useSpaceLayoutController(
  spacePk: string,
): SpaceLayoutController {
  const i18n = useSpaceLayoutI18n();
  const { data: space } = useSpaceById(spacePk);
  const { data: user } = useUserInfo();
  const { data: files } = useFileSpace(spacePk);
  const navigate = useNavigate();
  const popup = usePopup();
  const publishSpace = usePublishSpaceMutation();
  const startSpace = useStartSpaceMutation();
  // const finishSpace = useFinishSpaceMutation();
  const deleteSpace = useDeleteSpaceMutation();
  const participateSpace = useParticipateSpaceMutation();
  const updateSpaceTitle = useSpaceUpdateTitleMutation();

  const location = useLocation();

  const preTaskRequired =
    space.participated &&
    space.havePreTasks() &&
    !space.isAdmin() &&
    !space.isFinished;
  useEffect(() => {
    if (preTaskRequired) {
      navigate(route.spaceRequirements(spacePk), { replace: true });
    }
  }, [navigate, spacePk, space, preTaskRequired]);

  const role = useMemo(() => {
    if (space.isAdmin()) {
      // Admin
      return Role.Admin;
    }

    if (space.participated) {
      // Already participated
      return Role.Participant;
    }

    return Role.Viewer;
  }, [space]);

  let profile: SpaceParticipantProfileProps | null = null;
  if (user) {
    profile = {
      profileUrl: user.profile_url,
      displayName: user.nickname,
      username: user.username,
    };
  }
  if (role === Role.Participant && space.anonymous_participation) {
    profile = {
      profileUrl: space.participantProfileUrl,
      displayName: space.participantDisplayName,
      username: space.participantUsername,
    };
  }

  const menus: SideMenuProps[] = useMemo(() => {
    const baseMenus = [...BASE_MENUS, ...(SPACE_MENUS[space.spaceType] || [])];
    const items = preTaskRequired ? [...REQUIRE_MENUS] : baseMenus;
    if (space.isAdmin()) {
      items.push(...ADMIN_MENUS);
    }

    return items
      .filter((menu) => {
        if (!menu.visible) {
          return true;
        }
        return menu.visible(space);
      })
      .map((menu) => ({
        Icon: menu.Icon,
        to: menu.to(space),
        label: i18n[menu.label],
        tag: {
          label: menu.tag?.label,
          visible: menu.tag?.visible(space),
        },
      }));
  }, [space, i18n, preTaskRequired]);

  const timelineItems = useMemo(
    () => [
      {
        label: i18n.timeline_created_at_label,
        time: space.createdAt,
      },
    ],
    [space.createdAt, i18n.timeline_created_at_label],
  );
  const currentMenu = useMemo(() => {
    const currentPath = location.pathname;
    // 현재 경로와 매칭되는 메뉴 중 가장 긴 경로를 찾기 (더 구체적인 메뉴 우선)
    return menus
      .filter((menu) => currentPath.startsWith(menu.to))
      .sort((a, b) => b.to.length - a.to.length)[0];
  }, [menus, location.pathname]);

  const handlePublish = useCallback(
    async (publishType: PublishType) => {
      const visibility: SpaceVisibility = { type: publishType };
      try {
        await publishSpace.mutateAsync({
          spacePk: space.pk,
          visibility,
        });

        showSuccessToast(i18n.toast_publish_success);
      } catch (err) {
        logger.error('publish space failed: ', err);
        showErrorToast(i18n.toast_publish_failed);
      } finally {
        popup.close();
      }
    },
    [
      publishSpace,
      space.pk,
      i18n.toast_publish_success,
      i18n.toast_publish_failed,
      popup,
    ],
  );

  const handleActionPublish = useCallback(async () => {
    popup
      .open(<PublishSpaceModal onPublish={handlePublish} />)
      .withTitle(i18n.publish_space_title)
      .withoutBackdropClose();
  }, [popup, handlePublish, i18n.publish_space_title]);

  const handleStart = useCallback(async () => {
    try {
      await startSpace.mutateAsync({
        spacePk: space.pk,
        block: true,
      });

      showSuccessToast(i18n.toast_start_success);
    } catch (err) {
      logger.error('start space failed: ', err);
      showErrorToast(i18n.toast_start_failed);
    } finally {
      popup.close();
    }
  }, [
    space.pk,
    startSpace,
    popup,
    i18n.toast_start_success,
    i18n.toast_start_failed,
  ]);

  // const handleFinish = useCallback(async () => {
  //   try {
  //     await finishSpace.mutateAsync({
  //       spacePk: space.pk,
  //       block: true,
  //     });

  //     showSuccessToast(t('success_finish_space'));
  //   } catch (err) {
  //     logger.error('finish space failed: ', err);
  //     showErrorToast(t('failed_finish_space'));
  //   } finally {
  //     popup.close();
  //   }
  // }, [space.pk, finishSpace, popup, t]);

  const handleDelete = useCallback(async () => {
    try {
      await deleteSpace.mutateAsync({
        spacePk: space.pk,
      });

      navigate(route.home());
      showSuccessToast(i18n.toast_delete_success);
    } catch (err) {
      logger.error('delete space failed: ', err);
      showErrorToast(i18n.toast_delete_failed);
    } finally {
      popup.close();
    }
  }, [
    deleteSpace,
    i18n.toast_delete_failed,
    i18n.toast_delete_success,
    navigate,
    popup,
    space.pk,
  ]);

  const handleActionDelete = useCallback(async () => {
    popup
      .open(
        <SpaceDeleteModal
          spaceName={space.title}
          onDelete={handleDelete}
          onClose={() => {
            popup.close();
          }}
        />,
      )
      .withTitle(i18n.delete_space_title)
      .withoutBackdropClose();
  }, [popup, handleDelete, space.title, i18n.delete_space_title]);

  const handleActionStart = useCallback(async () => {
    popup
      .open(
        <SpaceStartModal
          onStarted={handleStart}
          onClose={() => {
            popup.close();
          }}
        />,
      )
      .withTitle(i18n.start_space_title)
      .withoutBackdropClose();
  }, [popup, handleStart, i18n.start_space_title]);

  const handleParticipate = useCallback(async () => {
    logger.debug('handleParticipate is called');

    try {
      // TODO: In the future, implement proper verifiable credential logic
      // For now, using empty string as the backend has a TODO comment
      const result = await participateSpace.mutateAsync({
        spacePk: space.pk,
        verifiablePresentation: '',
      });

      logger.debug('Participation successful:', result);
      showSuccessToast(i18n.toast_participate_success);
    } catch (error) {
      logger.error('Failed to participate in space:', error);
      showErrorToast(i18n.toast_participate_failed);
    }
  }, [
    participateSpace,
    space.pk,
    i18n.toast_participate_success,
    i18n.toast_participate_failed,
  ]);

  const adminActions = useMemo(() => {
    const ret: LayoutAction[] = [
      {
        label: i18n.action_admin_delete,
        onClick: handleActionDelete,
      },
    ];

    if (space.isInProgress && space.spaceType === SpaceType.Deliberation) {
      ret.unshift({
        label: i18n.action_admin_start,
        onClick: handleActionStart,
      });
    }

    // if (space.isStarted) {
    //   ret.unshift({
    //     label: t('finished'),
    //     onClick: handleFinish,
    //   });
    // }

    if (space.isDraft) {
      ret.unshift({
        label: i18n.action_admin_publish,
        onClick: handleActionPublish,
      });
    }

    return ret;
  }, [
    i18n.action_admin_delete,
    i18n.action_admin_start,
    i18n.action_admin_publish,
    handleActionDelete,
    space.isInProgress,
    space.spaceType,
    space.isDraft,
    handleActionStart,
    handleActionPublish,
  ]);

  const viewerActions = useMemo(() => {
    const ret: LayoutAction[] = [
      {
        label: i18n.action_viewer_participate,
        onClick: handleParticipate,
      },
      {
        label: i18n.action_viewer_credentials,
        onClick: () => {
          navigate(route.credentials());
        },
      },
    ];

    return ret;
  }, [
    i18n.action_viewer_participate,
    i18n.action_viewer_credentials,
    handleParticipate,
    navigate,
  ]);

  const participantActions = useMemo(() => {
    const ret: LayoutAction[] = [];

    return ret;
  }, []);

  const handleTitleChange = useCallback(
    async (title: string) => {
      logger.debug('Title change requested:', title);
      try {
        await updateSpaceTitle.mutateAsync({
          spacePk: space.pk,
          title,
        });
        showSuccessToast(i18n.toast_update_title_success);
      } catch (error) {
        logger.error('Failed to update space title', error);
        showErrorToast(`${i18n.toast_update_title_failed}: ${error}`);
      }
    },
    [
      i18n.toast_update_title_failed,
      i18n.toast_update_title_success,
      space.pk,
      updateSpaceTitle,
    ],
  );

  //

  return {
    space,
    user,
    files,
    menus,
    timelineItems,
    navigate,
    popup,
    profile,
    i18n,

    handleTitleChange,
    role,
    currentMenu,
    adminActions,
    viewerActions,
    participantActions,
  };
}
