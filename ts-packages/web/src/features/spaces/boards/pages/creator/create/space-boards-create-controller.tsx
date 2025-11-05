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
    public postPk: State<string | null>,
    public t: TFunction<'SpaceBoardsCreate', undefined>,

    public createSpacePosts: ReturnType<typeof useCreateSpacePostMutation>,
    public updateSpacePosts: ReturnType<typeof useUpdateSpacePostMutation>,
  ) {}

  handleContent = (htmlContents: string) => {
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
        });

        showSuccessToast('Success to update posts');
        this.navigate(route.spaceBoards(this.spacePk));
      } catch {
        showErrorToast('Failed to update posts.');
      }
    } else {
      try {
        await this.createSpacePosts.mutateAsync({
          spacePk: this.spacePk,
          title,
          htmlContents,
          categoryName,
          image,
        });

        showSuccessToast('Success to update posts');
        this.navigate(route.spaceBoards(this.spacePk));
      } catch {
        showErrorToast('Failed to update posts.');
      }
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
          logger.debug('Uploaded image URL:', uploadedUrl);

          this.image.set(uploadedUrl);
        }
      }
    } catch (error) {
      logger.error('Image upload failed:', error);
      showErrorToast('Failed to upload image');
    }
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

  const title = useState('');
  const htmlContents = useState('');
  const categoryName = useState('');
  const image = useState<string | null>(null);
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
        } catch (error) {
          logger.error('Failed to fetch post data:', error);
          showErrorToast('Failed to fetch post data');
        }
      }
    };

    initializePost();
  }, []);

  return new SpaceBoardsCreateController(
    spacePk,
    space,
    category,
    navigate,
    new State(title),
    new State(htmlContents),
    new State(categoryName),
    new State(image),
    new State(postPk),
    t,

    createSpacePosts,
    updateSpacePosts,
  );
}
