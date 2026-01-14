import { useState, useEffect, useRef } from 'react';
import { State } from '@/types/state';
import { useSpaceHomeData } from './use-space-home-data';
import { SideMenuProps } from '@/features/spaces/components/space-side-menu';
import { route } from '@/route';
import { Space } from '@/features/spaces/types/space';
import { Post, Settings } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { logger } from '@/lib/logger';
import { useSpaceUpdateContentMutation } from '@/features/spaces/hooks/use-space-update-content-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useSpaceUpdateTitleMutation } from '@/features/spaces/hooks/use-space-update-title-mutation';
import { sideMenusForSpaceType } from '@/features/spaces/utils/side-menus-for-space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import PublishSpaceModal from '@/features/spaces/modals/space-publish-modal';
import { usePublishSpaceMutation } from '@/features/spaces/hooks/use-publish-mutation';
import SpaceDeleteModal from '@/features/spaces/modals/space-delete-modal';
import { useDeleteSpaceMutation } from '@/features/spaces/hooks/use-delete-mutation';
import { NavigateFunction, useNavigate } from 'react-router';
import { UserDetailResponse } from '@/lib/api/ratel/users.v3';
import FileModel, {
  FileExtension,
  toFileExtension,
} from '@/features/spaces/files/types/file';
import { useSpaceUpdateFilesMutation } from '@/features/spaces/hooks/use-space-update-files-mutation';
import { useUpdateDraftImageMutation } from '@/features/posts/hooks/use-update-draft-image-mutation';
import { dataUrlToBlob, parseFileType } from '@/lib/file-utils';
import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { spacePkToPostPk } from '@/features/spaces/utils/partition-key-utils';
import { useParticipateSpaceMutation } from '@/features/spaces/hooks/use-participate-space-mutation';
import { SpaceType } from '@/features/spaces/types/space-type';
import SpaceStartModal from '@/features/spaces/modals/space-start-modal';
import { useStartSpaceMutation } from '@/features/spaces/hooks/use-start-mutation';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import useFileSpace from '@/features/spaces/files/hooks/use-file-space';
import SpaceAuthorizePopup from './space-authorize-popup';
import SpaceEndModal from '@/features/spaces/modals/space-end-modal';
import { useFinishSpaceMutation } from '@/features/spaces/hooks/use-finish-mutation';
import { Trophy } from '@/assets/icons/game';

export class SpaceHomeController {
  public space: Space;
  public user: UserDetailResponse | null;
  public saveHook?: () => Promise<void>;
  public publishHook?: () => Promise<void>;

  constructor(
    public navigate: NavigateFunction,
    public data: ReturnType<typeof useSpaceHomeData>,
    public state: State<boolean>,
    public t: TFunction<'Space'>,
    public updateSpaceContent: ReturnType<typeof useSpaceUpdateContentMutation>,
    public updateSpaceTitle: ReturnType<typeof useSpaceUpdateTitleMutation>,
    public updateSpaceFiles: ReturnType<typeof useSpaceUpdateFilesMutation>,
    public editState: State<boolean>,
    public saveState: State<boolean>,
    public popup: ReturnType<typeof usePopup>,
    public publishSpace: ReturnType<typeof usePublishSpaceMutation>,
    public startSpace: ReturnType<typeof useStartSpaceMutation>,
    public finishSpace: ReturnType<typeof useFinishSpaceMutation>,
    public deleteSpace: ReturnType<typeof useDeleteSpaceMutation>,
    public image: State<string | null>,
    public hasFiles: boolean,
    public files: State<FileModel[]>,
    public updateDraftImage: ReturnType<
      typeof useUpdateDraftImageMutation
    >['mutateAsync'],
    public participateSpace: ReturnType<typeof useParticipateSpaceMutation>,
    public hiding: State<boolean>,
  ) {
    this.space = this.data.space.data;
    this.user = this.data.user.data;
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
        Icon: Post,
        to: route.spaceByType(this.space.spaceType, this.space.pk),
        label: this.t('menu_overview'),
      },
    ];

    const hasFiles = this.hasFiles;

    sideMenusForSpaceType[this.space.spaceType]?.forEach((menu) => {
      let visible = !menu.visible;

      if (menu.label === 'menu_files') {
        visible = visible && (hasFiles || this.space.isAdmin());
      }

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

    // It seems like admins shouldn't be able to change settings after a space is published.
    // If a change is made to the settings, the anomyous attribute can also be changed, but in this case, matching between participating and non-participating users may not be possible.
    if (this.space.isAdmin()) {
      menus = menus.concat(this.adminMenus);
    }

    return menus;
  }

  get isAdmin() {
    return this.space.isAdmin();
  }

  get adminMenus(): SideMenuProps[] {
    const menus = [
      {
        Icon: Settings,
        to: route.spaceSetting(this.space.pk),
        label: this.t('menu_admin_settings'),
      },
    ];
    // menus.push({
    //   Icon: Trophy,
    //   to: route.spaceReward(this.space.pk),
    //   label: this.t('menu_rewards'),
    // });
    return menus;
  }

  handleRemovePdf = (index: number) => {
    const prev = this.files?.get?.() ?? [];
    if (index < 0 || index >= prev.length) return;

    const removed = prev[index];
    try {
      if (removed?.url && removed.url.startsWith('blob:')) {
        URL.revokeObjectURL(removed.url);
      }
    } catch (e) {
      logger.error('remove pdf error: ', e);
    }

    const next = [...prev.slice(0, index), ...prev.slice(index + 1)];
    this.files.set(next);
  };

  handlePdfUpload = async (fileList: FileList | File[]) => {
    const maxSizeMB = 50;
    const files = Array.from(fileList);

    if (files.length === 0) return;

    for (const f of files) {
      if (f.type !== 'application/pdf') {
        showErrorToast('only PDF files can uploaded');
        return;
      }
      if (f.size > maxSizeMB * 1024 * 1024) {
        showErrorToast(`Each file must be less than ${maxSizeMB}MB.`);
        return;
      }
    }

    try {
      const presign = await getPutObjectUrl(
        files.length,
        parseFileType('application/pdf'),
      );
      const presigned = presign?.presigned_uris ?? [];
      const uris = presign?.uris ?? [];

      if (presigned.length !== files.length || uris.length !== files.length) {
        showErrorToast('Failed to issue upload URL.');
        return;
      }

      await Promise.all(
        files.map((file, i) =>
          fetch(presigned[i], {
            method: 'PUT',
            headers: { 'Content-Type': 'application/pdf' },
            body: file,
          }),
        ),
      );

      const newModels: FileModel[] = files.map((file, i) => ({
        id: crypto.randomUUID(),
        name: file.name,
        size: `${(file.size / 1024 / 1024).toFixed(2)} MB`,
        ext: FileExtension.PDF,
        url: uris[i],
      }));

      this.files.set([...this.files.get(), ...newModels]);
      showSuccessToast('Complete to PDF upload');
    } catch (error) {
      logger.error('PDF upload failed:', error);
      showErrorToast('Failed to PDF upload');
    }
  };

  handleAddFile = async (file: FileModel) => {
    const files = this.space.files ?? [];
    files.push(file);

    this.handleUploadFiles(files);
  };

  handleRemoveFile = async (index: number) => {
    const newFiles = this.space.files.filter((_, i) => i !== index);
    this.handleUploadFiles(newFiles);
  };

  handleUploadFiles = async (files: FileModel[]) => {
    try {
      await this.updateSpaceFiles.mutateAsync({
        spacePk: this.space.pk,
        files,
      });
      showSuccessToast('Success to update space files');
    } catch (error) {
      logger.error('Failed to update space files', error);
      showErrorToast(`Failed to update space files: ${error}`);
    }
  };

  handleChange = async (text: string) => {
    try {
      await this.updateSpaceContent.mutateAsync({
        spacePk: this.space.pk,
        content: text,
      });
      await this.updateSpaceFiles.mutateAsync({
        spacePk: this.space.pk,
        files: this.files.get(),
      });
      showSuccessToast('Success to update space content');
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
      showSuccessToast('Success to update space title');
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

    try {
      this.publishSpace.mutateAsync({
        spacePk: this.space.pk,
        visibility,
      });

      showSuccessToast(this.t('success_publish_space'));
    } catch (err) {
      logger.error('publish space failed: ', err);
      showErrorToast(this.t('failed_publish_space'));
    } finally {
      this.popup.close();
    }
  };

  handleStart = async () => {
    try {
      this.startSpace.mutateAsync({
        spacePk: this.space.pk,
        block: true,
      });

      showSuccessToast(this.t('success_start_space'));
    } catch (err) {
      logger.error('start space failed: ', err);
      showErrorToast(this.t('failed_start_space'));
    } finally {
      this.popup.close();
    }

    this.popup.close();
  };

  handleFinish = async () => {
    try {
      this.finishSpace.mutateAsync({
        spacePk: this.space.pk,
        block: true,
      });

      showSuccessToast(this.t('success_finish_space'));
    } catch (err) {
      logger.error('finish space failed: ', err);
      showErrorToast(this.t('failed_finish_space'));
    } finally {
      this.popup.close();
    }

    this.popup.close();
  };

  handleDelete = async () => {
    if (this.publishHook) {
      this.publishHook();
    }

    try {
      this.deleteSpace.mutateAsync({
        spacePk: this.space.pk,
      });

      this.navigate(route.home());
      showSuccessToast(this.t('success_delete_space'));
    } catch (err) {
      logger.error('delete space failed: ', err);
      showErrorToast(this.t('failed_delete_space'));
    } finally {
      this.popup.close();
    }

    this.popup.close();
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

  handleActionPrivate = async () => {
    this.publishSpace.mutateAsync({
      spacePk: this.space.pk,
      visibility: { type: 'PRIVATE' },
    });
  };

  handleActionPublic = async () => {
    this.publishSpace.mutateAsync({
      spacePk: this.space.pk,
      visibility: { type: 'PUBLIC' },
    });
  };

  handleActionStart = async () => {
    logger.debug('Action start triggered');

    this.popup
      .open(
        <SpaceStartModal
          onStarted={this.handleStart}
          onClose={() => {
            this.popup.close();
          }}
        />,
      )
      .withTitle(this.t('start_space'))
      .withoutBackdropClose();
  };

  handleActionFinish = async () => {
    logger.debug('Action end triggered');

    this.popup
      .open(
        <SpaceEndModal
          onEnded={this.handleFinish}
          onClose={() => {
            this.popup.close();
          }}
        />,
      )
      .withTitle(this.t('end_space'))
      .withoutBackdropClose();
  };

  handleImageUpload = async (imageUrl: string) => {
    const postPk = spacePkToPostPk(this.space.pk);
    if (!postPk) return;

    try {
      const mime = imageUrl.match(/^data:([^;]+);base64,/);
      if (mime && mime[1]) {
        const res = await getPutObjectUrl(1, parseFileType(mime[1]));

        if (res && res.presigned_uris?.length > 0 && res.uris?.length > 0) {
          const blob = await dataUrlToBlob(imageUrl);
          await fetch(res.presigned_uris[0], {
            method: 'PUT',
            headers: {
              'Content-Type': mime[1],
            },
            body: blob,
          });
          const uploadedUrl = res.uris[0];
          logger.debug('Uploaded image URL:', uploadedUrl, postPk);
          if (uploadedUrl) {
            await this.updateDraftImage({
              postPk: postPk,
              image: uploadedUrl,
            });
          }

          this.image.set(uploadedUrl);
        }
      }
    } catch (error) {
      logger.error('Image upload failed:', error);
      showErrorToast('Failed to upload image');
    }
  };

  handleRemoveImage = async () => {
    const postPk = spacePkToPostPk(this.space.pk);
    if (!postPk) return;

    await this.updateDraftImage({
      postPk: postPk,
      image: null,
    });
    this.image.set(null);
  };

  handleParticipate = async () => {
    logger.debug('handleParticipate is called');

    try {
      // TODO: In the future, implement proper verifiable credential logic
      // For now, using empty string as the backend has a TODO comment
      const result = await this.participateSpace.mutateAsync({
        spacePk: this.space.pk,
        verifiablePresentation: '',
      });

      logger.debug('Participation successful:', result);
      showSuccessToast(this.t('success_participate_space'));
    } catch (error) {
      logger.error('Failed to participate in space:', error);
      showErrorToast(this.t('failed_participate_space'));
    }
  };

  canParticipate() {
    // User can participate if:
    // 1. User is authenticated
    // 2. User is not already a space admin
    // 3. User is not already an anonymous space participant
    if (this.space.participated) {
      return false;
    }

    if (this.isAdmin) {
      return false;
    }

    return this.space.canParticipate;
  }

  get actions() {
    if (this.isAdmin) {
      return this.adminActions;
    } else if (
      this.space.shouldParticipateManually() &&
      this.canParticipate() &&
      this.space.status == SpaceStatus.InProgress
      // check already joined
    ) {
      return this.viewerActions;
    }

    return this.participantActions;
  }

  get viewerActions() {
    const ret = [
      {
        label: this.t('action_participate'),
        onClick: this.handleParticipate,
      },
    ];

    return ret;
  }

  get adminActions() {
    const ret = [
      {
        label: this.t('delete'),
        onClick: this.handleActionDelete,
      },
    ];

    if (
      this.space.isInProgress &&
      this.space.isPublic &&
      this.space.change_visibility
    ) {
      ret.unshift({
        label: this.t('change_private'),
        onClick: this.handleActionPrivate,
      });
    }

    if (
      this.space.isInProgress &&
      !this.space.isPublic &&
      this.space.change_visibility
    ) {
      ret.unshift({
        label: this.t('change_public'),
        onClick: this.handleActionPublic,
      });
    }

    if (
      this.space.isInProgress &&
      this.space.spaceType === SpaceType.Deliberation
    ) {
      ret.unshift({
        label: this.t('started'),
        onClick: this.handleActionStart,
      });
    }

    if (this.space.isStarted) {
      ret.unshift({
        label: this.t('finished'),
        onClick: this.handleActionFinish,
      });
    }

    // if (this.space.isStarted) {
    //   ret.unshift({
    //     label: this.t('finished'),
    //     onClick: this.handleActionFinished,
    //   });
    // }

    if (this.space.isDraft) {
      ret.unshift({
        label: this.t('publish'),
        onClick: this.handleActionPublish,
      });
    }

    return ret;
  }

  get participantActions() {
    const ret = [];

    return ret;
  }
}

export function useSpaceHomeController(spacePk: string) {
  const data = useSpaceHomeData(spacePk);
  const state = useState(false);
  const { t } = useTranslation('Space');
  const navigate = useNavigate();
  const fileData = useFileSpace(spacePk);
  const updateSpaceContent = useSpaceUpdateContentMutation();
  const updateSpaceTitle = useSpaceUpdateTitleMutation();
  const updateSpaceFiles = useSpaceUpdateFilesMutation();
  const publishSpace = usePublishSpaceMutation();
  const startSpace = useStartSpaceMutation();
  const finishSpace = useFinishSpaceMutation();
  const deleteSpace = useDeleteSpaceMutation();
  const { mutateAsync: updateDraftImage } = useUpdateDraftImageMutation();
  const participateSpace = useParticipateSpaceMutation();

  const hasFiles = fileData.data.files.length !== 0;

  const edit = useState(false);
  const save = useState(false);
  const popup = usePopup();
  const image = useState<string | null>(null);
  const files = useState<FileModel[]>([]);
  const filesInitializedRef = useRef(false);

  const hiding = useState(false);

  // Initialize image from space data
  useEffect(() => {
    if (
      data.space.data &&
      data.space.data.urls &&
      data.space.data.urls.length > 0
    ) {
      image[1](data.space.data.urls[0]);
    }
  }, [data.space.data, image]);

  useEffect(() => {
    const remote = data.space.data?.files ?? [];
    if (!data.space.isSuccess) return;
    if (filesInitializedRef.current) return;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const mapped: FileModel[] = remote.map((f: any) => ({
      name: f.name ?? '',
      size:
        typeof f.size === 'string'
          ? f.size
          : `${Math.max(0, Number(f.size || 0) / 1024 / 1024).toFixed(2)} MB`,
      ext: toFileExtension(f.ext),
      url: f.url ?? null,
    }));

    files[1](mapped);
    filesInitializedRef.current = true;
  }, [data.space.isSuccess, data.space.data?.files]);

  const participationAttemptedRef = useRef(false);

  useEffect(() => {
    if (participationAttemptedRef.current || participateSpace.isPending) {
      return;
    }

    const space = data.space.data;

    if (!space) return;

    const shouldAutoParticipate = space.canParticipate;

    if (!shouldAutoParticipate) return;

    participationAttemptedRef.current = true;

    (async () => {
      try {
        await participateSpace.mutateAsync({
          spacePk,
          verifiablePresentation: '',
        });
      } catch (err) {
        logger.debug('auto participate failed: ', err);
        console.log('auto participate failed: ', err);

        popup.open(<SpaceAuthorizePopup />).withTitle(t('authorize_title'));
      }
    })();
  }, [
    spacePk,
    data.space.data?.pk,
    data.space.data?.canParticipate,
    data.space.data?.status,
  ]);

  return new SpaceHomeController(
    navigate,
    data,
    new State(state),
    t,
    updateSpaceContent,
    updateSpaceTitle,
    updateSpaceFiles,
    new State(edit),
    new State(save),
    popup,
    publishSpace,
    startSpace,
    finishSpace,
    deleteSpace,
    new State(image),
    hasFiles,
    new State(files),
    updateDraftImage,
    participateSpace,
    new State(hiding),
  );
}
