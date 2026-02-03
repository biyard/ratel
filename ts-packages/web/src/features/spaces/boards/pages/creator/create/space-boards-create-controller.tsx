import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { dataUrlToBlob, parseFileType } from '@/lib/file-utils';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { State } from '@/types/state';
import { useEffect, useRef, useState } from 'react';
import useSpaceCategory from '../../../hooks/use-space-category';
import { SpaceCategory } from '../../../types/space-category';
import { useCreateSpacePostMutation } from '../../../hooks/use-create-space-post-mutation';
import { useNavigate, useSearchParams } from 'react-router';
import { route } from '@/route';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useUpdateSpacePostMutation } from '../../../hooks/use-update-space-post-mutation';
import { getSpacePost } from '../../../hooks/use-space-post';
import FileModel, { FileExtension } from '@/features/spaces/files/types/file';

export class SpaceBoardsCreateController {
  constructor(
    public spacePk: string,
    public space: Space,
    public category: SpaceCategory,
    public navigate: ReturnType<typeof useNavigate>,
    public title: State<string>,
    public htmlContents: State<string>,
    public categoryName: State<string>,
    public image: State<string | null>,
    public files: State<FileModel[]>,
    public startedAt: State<number>,
    public endedAt: State<number>,
    public postPk: State<string | null>,
    public t: TFunction<'SpaceBoardsCreate', undefined>,

    public createSpacePosts: ReturnType<typeof useCreateSpacePostMutation>,
    public updateSpacePosts: ReturnType<typeof useUpdateSpacePostMutation>,
  ) {}

  handleContent = async (htmlContents: string) => {
    this.htmlContents.set(htmlContents);
  };

  handleTitle = (title: string) => {
    this.title.set(title);
  };

  handleCategoryName = (categoryName: string) => {
    this.categoryName.set(categoryName);
  };

  handleCancel = () => {
    this.navigate(route.spaceBoards(this.spacePk));
  };

  handleSubmit = async () => {
    const title = this.title.get();
    const htmlContents = this.htmlContents.get();
    const categoryName = this.categoryName.get();
    const image = this.image.get();

    if (this.postPk.get() && this.postPk.get() != null) {
      try {
        await this.updateSpacePosts.mutateAsync({
          spacePk: this.spacePk,
          postPk: this.postPk.get(),
          title,
          htmlContents,
          categoryName,
          image,
          files: this.files.get() ?? [],

          startedAt: this.startedAt.get(),
          endedAt: this.endedAt.get(),
        });

        showSuccessToast('Success to update posts');
        this.navigate(route.spaceBoards(this.spacePk));
      } catch {
        showErrorToast('Failed to update posts.');
      }
    } else {
      try {
        const filesToSend = this.files.get() ?? [];

        await this.createSpacePosts.mutateAsync({
          spacePk: this.spacePk,
          title,
          htmlContents,
          categoryName,
          image,
          files: filesToSend,

          startedAt: this.startedAt.get(),
          endedAt: this.endedAt.get(),
        });

        showSuccessToast('Success to update posts');
        this.navigate(route.spaceBoards(this.spacePk));
      } catch {
        showErrorToast('Failed to update posts.');
      }
    }
  };

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

  handleImageUpload = async (imageUrl: string) => {
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
          this.image.set(uploadedUrl);
        }
      }
    } catch (error) {
      logger.error('Image upload failed:', error);
      showErrorToast('Failed to upload image');
    }
  };

  uploadAsset = async (file: File): Promise<{ url: string }> => {
    try {
      const res = await getPutObjectUrl(1, parseFileType(file.type));

      if (!res || !res.presigned_uris?.[0] || !res.uris?.[0]) {
        throw new Error('Failed to get presigned URL');
      }

      await fetch(res.presigned_uris[0], {
        method: 'PUT',
        headers: {
          'Content-Type': file.type,
        },
        body: file,
      });

      const uploadedUrl = res.uris[0];
      return { url: uploadedUrl };
    } catch (error) {
      logger.error('Asset upload failed:', error);
      showErrorToast('Failed to upload asset');
      throw error;
    }
  };

  handleTimeRange = async (started_at: number, ended_at: number) => {
    if (started_at >= ended_at) {
      showErrorToast(this.t('invalid_time'));
      return;
    }

    this.startedAt.set(started_at);
    this.endedAt.set(ended_at);
  };

  handleRemoveImage = async () => {
    this.image.set(null);
  };
}

export function useSpaceBoardsCreateController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: category } = useSpaceCategory(spacePk);
  const { t } = useTranslation('SpaceBoardsCreate');
  const [searchParams] = useSearchParams();
  const postPkParam = searchParams.get('post-pk');
  const now = new Date().getTime();

  const title = useState('');
  const htmlContents = useState('');
  const categoryName = useState('');
  const startedAt = useState(now);
  const endedAt = useState(now);
  const image = useState<string | null>(null);
  const files = useState<FileModel[]>([]);
  const postPk = useState<string | null>(postPkParam);

  const createSpacePosts = useCreateSpacePostMutation();
  const updateSpacePosts = useUpdateSpacePostMutation();
  const navigate = useNavigate();

  const initializedRef = useRef(false);

  const controller = new SpaceBoardsCreateController(
    spacePk,
    space,
    category,
    navigate,
    new State(title),
    new State(htmlContents),
    new State(categoryName),
    new State(image),
    new State(files),
    new State(startedAt),
    new State(endedAt),
    new State(postPk),
    t,

    createSpacePosts,
    updateSpacePosts,
  );

  useEffect(() => {
    const initializePost = async () => {
      if (initializedRef.current) {
        logger.debug('Skipping duplicate initialization (React StrictMode)');
        return;
      }
      initializedRef.current = true;

      if (postPkParam) {
        try {
          const post = await getSpacePost(spacePk, postPkParam);
          controller.title.set(post.title);
          controller.htmlContents.set(post.html_contents);
          controller.categoryName.set(post.category_name);
          controller.image.set(post.urls.length == 0 ? '' : post.urls[0]);
          controller.files.set(post.files.length == 0 ? [] : post.files);
          controller.startedAt.set(post.started_at ?? now);
          controller.endedAt.set(post.ended_at ?? now);
        } catch (error) {
          logger.error('Failed to fetch post data:', error);
          showErrorToast('Failed to fetch post data');
        }
      }
    };

    initializePost();
  }, []);

  return controller;
}
