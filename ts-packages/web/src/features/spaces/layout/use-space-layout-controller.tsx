import { useMemo, useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import { Post, Settings } from '@/components/icons';
import { route } from '@/route';
import { SPACE_MENUS } from './space-menus';
import { SideMenu } from './types/space-menu';
import { useSpaceById } from '../hooks/use-space-by-id';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import useFileSpace from '../files/hooks/use-file-space';
import { SpaceType } from '../types/space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import { usePublishSpaceMutation } from '../hooks/use-publish-mutation';
import { useStartSpaceMutation } from '../hooks/use-start-mutation';
import { useFinishSpaceMutation } from '../hooks/use-finish-mutation';
import { useDeleteSpaceMutation } from '../hooks/use-delete-mutation';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { Space } from '../types/space';
import { UserDetailResponse } from '@/lib/api/ratel/users.v3';
import { FileResponse } from '../files/types/file-response';

export type SideMenuProps = {
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  to: string;
  label: string;
};

export interface SpaceLayoutController {
  space: Space;
  user: UserDetailResponse | null;
  files: FileResponse;
  menus: SideMenuProps[];
  adminMenus: SideMenuProps[];
  timelineItems: Array<{ label: string; time: number }>;
  navigate: ReturnType<typeof useNavigate>;
  popup: ReturnType<typeof usePopup>;
  t: ReturnType<typeof useTranslation>['t'];
  handlePublish: (publishType: string) => Promise<void>;
  handleStart: () => Promise<void>;
  handleFinish: () => Promise<void>;
  handleDelete: () => Promise<void>;
  handleActionPrivate: () => Promise<void>;
  handleActionPublic: () => Promise<void>;
  adminActions: Array<{ label: string; onClick: () => Promise<void> }>;
  actions: Array<{ label: string; onClick: () => Promise<void> }>;
  shouldShowLayout: boolean;
  isAdmin: boolean;
  handleTitleChange: (title: string) => void;
  // Requirements management
  requirementIndex: number;
  setRequirementIndex: (index: number) => void;
  handleNextRequirement: () => void;
  shouldHideLayout: boolean;
  setShouldHideLayout: (hide: boolean) => void;
}

export function useSpaceLayoutController(
  spacePk: string,
): SpaceLayoutController {
  const { t } = useTranslation('Space');
  const { data: space } = useSpaceById(spacePk);
  const { data: user } = useSuspenseUserInfo();
  const { data: files } = useFileSpace(spacePk);
  const navigate = useNavigate();
  const popup = usePopup();
  const publishSpace = usePublishSpaceMutation();
  const startSpace = useStartSpaceMutation();
  const finishSpace = useFinishSpaceMutation();
  const deleteSpace = useDeleteSpaceMutation();

  // Requirements management state
  const [requirementIndex, setRequirementIndex] = useState(() =>
    space.requirements.findIndex((el) => !el.responded),
  );
  const [shouldHideLayout, setShouldHideLayout] = useState(false);

  const isMenuVisible = useCallback(
    (menu: SideMenu): boolean => {
      if (menu.label === 'menu_files') {
        return files.files.length !== 0 || space.isAdmin();
      }

      if (menu.visible) {
        return menu.visible(space);
      }

      return true;
    },
    [space, files],
  );

  const adminMenus: SideMenuProps[] = useMemo(
    () => [
      {
        Icon: Settings,
        to: route.spaceSetting(space.pk),
        label: t('menu_admin_settings'),
      },
    ],
    [space.pk, t],
  );

  const menus: SideMenuProps[] = useMemo(() => {
    const baseMenus: SideMenuProps[] = [
      {
        Icon: Post,
        to: route.spaceByType(space.spaceType, space.pk),
        label: t('menu_overview'),
      },
    ];

    const typeMenus = SPACE_MENUS[space.spaceType] || [];
    const filteredMenus = typeMenus
      .filter((menu) => isMenuVisible(menu))
      .map((menu) => ({
        Icon: menu.Icon,
        to: typeof menu.to === 'function' ? menu.to(space) : menu.to,
        label: t(menu.label),
      }));

    const allMenus = [...baseMenus, ...filteredMenus];

    if (space.isAdmin()) {
      return [...allMenus, ...adminMenus];
    }

    return allMenus;
  }, [space, t, isMenuVisible, adminMenus]);

  const timelineItems = useMemo(
    () => [
      {
        label: t('timeline_created_at_label'),
        time: space.createdAt,
      },
    ],
    [space.createdAt, t],
  );

  const handlePublish = useCallback(
    async (publishType: string) => {
      logger.debug('Publishing space with type:', publishType);

      try {
        await publishSpace.mutateAsync({
          spacePk: space.pk,
          visibility: space.visibility,
        });

        showSuccessToast(t('success_publish_space'));
      } catch (err) {
        logger.error('publish space failed: ', err);
        showErrorToast(t('failed_publish_space'));
      } finally {
        popup.close();
      }
    },
    [publishSpace, space.pk, space.visibility, t, popup],
  );

  const handleStart = useCallback(async () => {
    try {
      await startSpace.mutateAsync({
        spacePk: space.pk,
        block: true,
      });

      showSuccessToast(t('success_start_space'));
    } catch (err) {
      logger.error('start space failed: ', err);
      showErrorToast(t('failed_start_space'));
    } finally {
      popup.close();
    }
  }, [space.pk, startSpace, popup, t]);

  const handleFinish = useCallback(async () => {
    try {
      await finishSpace.mutateAsync({
        spacePk: space.pk,
        block: true,
      });

      showSuccessToast(t('success_finish_space'));
    } catch (err) {
      logger.error('finish space failed: ', err);
      showErrorToast(t('failed_finish_space'));
    } finally {
      popup.close();
    }
  }, [space.pk, finishSpace, popup, t]);

  const handleDelete = useCallback(async () => {
    try {
      await deleteSpace.mutateAsync({
        spacePk: space.pk,
      });

      navigate(route.home());
      showSuccessToast(t('success_delete_space'));
    } catch (err) {
      logger.error('delete space failed: ', err);
      showErrorToast(t('failed_delete_space'));
    } finally {
      popup.close();
    }
  }, [space.pk, deleteSpace, navigate, popup, t]);

  const handleActionPrivate = useCallback(async () => {
    await publishSpace.mutateAsync({
      spacePk: space.pk,
      visibility: { type: 'PRIVATE' },
    });
  }, [space.pk, publishSpace]);

  const handleActionPublic = useCallback(async () => {
    await publishSpace.mutateAsync({
      spacePk: space.pk,
      visibility: { type: 'PUBLIC' },
    });
  }, [space.pk, publishSpace]);

  const adminActions = useMemo(() => {
    const ret: Array<{ label: string; onClick: () => Promise<void> }> = [
      {
        label: t('delete'),
        onClick: handleDelete,
      },
    ];

    if (space.isInProgress && space.isPublic && space.change_visibility) {
      ret.unshift({
        label: t('change_private'),
        onClick: handleActionPrivate,
      });
    }

    if (space.isInProgress && !space.isPublic && space.change_visibility) {
      ret.unshift({
        label: t('change_public'),
        onClick: handleActionPublic,
      });
    }

    if (space.isInProgress && space.spaceType === SpaceType.Deliberation) {
      ret.unshift({
        label: t('started'),
        onClick: handleStart,
      });
    }

    if (space.isStarted) {
      ret.unshift({
        label: t('finished'),
        onClick: handleFinish,
      });
    }

    return ret;
  }, [
    space,
    t,
    handleDelete,
    handleActionPrivate,
    handleActionPublic,
    handleStart,
    handleFinish,
  ]);

  const handleNextRequirement = useCallback(() => {
    logger.debug(
      'handleNextRequirement called, current index:',
      requirementIndex,
    );

    const currentIdx = requirementIndex;

    if (currentIdx >= space.requirements.length - 1) {
      setRequirementIndex(currentIdx + 1);
    } else {
      // Navigate to next requirement or complete
      setRequirementIndex(currentIdx + 1);
    }
  }, [requirementIndex, space.requirements.length]);

  const handleTitleChange = useCallback((title: string) => {
    // TODO: Implement title change logic
    logger.debug('Title change requested:', title);
  }, []);

  return {
    space,
    user,
    files,
    menus,
    adminMenus,
    timelineItems,
    navigate,
    popup,
    t,
    handlePublish,
    handleStart,
    handleFinish,
    handleDelete,
    handleActionPrivate,
    handleActionPublic,
    adminActions,
    actions: space.isAdmin() ? adminActions : [],
    shouldShowLayout: !shouldHideLayout,
    isAdmin: space.isAdmin(),
    handleTitleChange,
    requirementIndex,
    setRequirementIndex,
    handleNextRequirement,
    shouldHideLayout,
    setShouldHideLayout,
  };
}
