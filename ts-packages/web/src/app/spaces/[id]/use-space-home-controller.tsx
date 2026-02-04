import { useState, useEffect, useRef } from 'react';
import { State } from '@/types/state';
import { useSpaceHomeData } from './use-space-home-data';
import { Space } from '@/features/spaces/types/space';
import { logger } from '@/lib/logger';
import { useSpaceUpdateContentMutation } from '@/features/spaces/hooks/use-space-update-content-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useSpaceUpdateTitleMutation } from '@/features/spaces/hooks/use-space-update-title-mutation';
import { usePopup } from '@/lib/contexts/popup-service';

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
import { type I18nSpaceHome, useSpaceHomeI18n } from './space-home-i18n';

export class SpaceHomeController {
  public space: Space;
  public user: UserDetailResponse | null;
  public saveHook?: () => Promise<void>;
  public publishHook?: () => Promise<void>;

  constructor(
    public navigate: NavigateFunction,
    public data: ReturnType<typeof useSpaceHomeData>,
    public state: State<boolean>,
    public i18n: I18nSpaceHome,
    public updateSpaceContent: ReturnType<typeof useSpaceUpdateContentMutation>,
    public updateSpaceTitle: ReturnType<typeof useSpaceUpdateTitleMutation>,
    public updateSpaceFiles: ReturnType<typeof useSpaceUpdateFilesMutation>,
    public editState: State<boolean>,
    public saveState: State<boolean>,
    public popup: ReturnType<typeof usePopup>,
    public image: State<string | null>,
    public files: State<FileModel[]>,
    public updateDraftImage: ReturnType<
      typeof useUpdateDraftImageMutation
    >['mutateAsync'],
  ) {
    this.space = this.data.space.data;
    this.user = this.data.user.data;
  }

  get isAdmin() {
    return this.space.isAdmin();
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
        showErrorToast(this.i18n.onlyPdfFiles);
        return;
      }
      if (f.size > maxSizeMB * 1024 * 1024) {
        showErrorToast(this.i18n.fileSizeLimit);
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
        showErrorToast(this.i18n.failedIssueUploadUrl);
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
      showSuccessToast(this.i18n.completePdfUpload);
    } catch (error) {
      logger.error('PDF upload failed:', error);
      showErrorToast(this.i18n.failedPdfUpload);
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
      showSuccessToast(this.i18n.successUpdateFiles);
    } catch (error) {
      logger.error('Failed to update space files', error);
      showErrorToast(`${this.i18n.failedUpdateFiles}: ${error}`);
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
      showSuccessToast(this.i18n.successUpdateContent);
    } catch (error) {
      logger.error('Failed to update space content', error);
      showErrorToast(`${this.i18n.failedUpdateContent}: ${error}`);
    }
  };

  handleTitleChange = async (title: string) => {
    logger.debug('Title change requested:', title);
    try {
      await this.updateSpaceTitle.mutateAsync({
        spacePk: this.space.pk,
        title,
      });
      showSuccessToast(this.i18n.successUpdateTitle);
    } catch (error) {
      logger.error('Failed to update space title', error);
      showErrorToast(`${this.i18n.failedUpdateTitle}: ${error}`);
    }
  };

  handleShare = async () => {
    logger.error('handleShare not implemented');
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
      showErrorToast(this.i18n.failedUploadImage);
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
}

export function useSpaceHomeController(spacePk: string) {
  const data = useSpaceHomeData(spacePk);
  const state = useState(false);
  const i18n = useSpaceHomeI18n();
  // const { t } = useTranslation('SpaceHome');
  const navigate = useNavigate();
  const updateSpaceContent = useSpaceUpdateContentMutation();
  const updateSpaceTitle = useSpaceUpdateTitleMutation();
  const updateSpaceFiles = useSpaceUpdateFilesMutation();

  const { mutateAsync: updateDraftImage } = useUpdateDraftImageMutation();

  const edit = useState(false);
  const save = useState(false);
  const popup = usePopup();
  const image = useState<string | null>(null);
  const files = useState<FileModel[]>([]);

  // FIXME: filesInitializedRef is Not needed.
  const filesInitializedRef = useRef(false);

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
      id: f.id ?? '',
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

  return new SpaceHomeController(
    navigate,
    data,
    new State(state),
    i18n,
    updateSpaceContent,
    updateSpaceTitle,
    updateSpaceFiles,
    new State(edit),
    new State(save),
    popup,

    new State(image),
    new State(files),
    updateDraftImage,
  );
}
