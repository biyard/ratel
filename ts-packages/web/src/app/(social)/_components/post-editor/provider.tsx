import { ArtworkTrait, ArtworkTraitDisplayType } from '@/lib/api/models/feeds';
import { checkString } from '@/lib/string-filter-utils';
import { useRouter, usePathname } from 'next/navigation';
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { route } from '@/route';
import { dataUrlToBlob, parseFileType } from '@/lib/file-utils';
import { AssetPresignedUris } from '@/lib/api/models/asset-presigned-uris';
import { getPost, Post, PostType as PT } from '@/lib/api/ratel/posts.v3';
import { useUpdateDraftMutation } from './use-update-draft-mutation';
import { useUpdateDraftImageMutation } from './use-update-draft-image-mutation';
import { usePublishDraftMutation } from './use-publish-draft';

export enum Status {
  Idle = 'Idle',
  Loading = 'Loading',
  Saving = 'Saving',
  Publishing = 'Publishing',
}

export enum PostType {
  Artwork = 'Artwork',
  General = 'General',
}

const AUTO_SAVE_DELAY = 5000; // ms
export interface PostEditorContextType {
  openPostEditorPopup: (postId: string) => Promise<void>;
  // openPostEditorPopupWithState: (id: number) => Promise<void>;

  expand: boolean;
  toggleExpand: () => void;
  postType: PostType;
  updatePostType: (type: PostType) => void;

  title: string;
  updateTitle: (title: string) => void;
  content: string | null;
  updateContent: (content: string) => void;
  image: string | null;
  updateImage: (image: string | null) => void;

  traits: ArtworkTrait[];
  updateTrait: (trait_type: string, value: string) => void;

  handleUpdate: () => Promise<void>;

  close: boolean;
  setClose: (value: boolean) => void;
  isSubmitDisabled: boolean;
  status: Status;
}

export const PostDraftContext = createContext<
  PostEditorContextType | undefined
>(undefined);

export function PostEditorProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  /*
    If Team is selected, use `team_id` as targetId
    Otherwise, use `user_id` as targetId
    if Not Logged in, use `0` as targetId
  */

  const pathname = usePathname();

  //Interal State
  const router = useRouter();
  const [close, setClose] = useState(true);
  const [expand, setExpand] = useState(false);
  const [status, setStatus] = useState<Status>(Status.Idle);
  const [feed, setFeed] = useState<Post | null>(null);
  const [postType, setPostType] = useState<PostType>(PostType.General);
  const [isModified, setIsModified] = useState(false);

  //State
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [image, setImage] = useState<string | null>(null);
  // const [artistName, setArtistName] = useState<string | null>(null);
  // const [backgroundColor, setBackgroundColor] = useState<string>('#ffffff');
  // const [size, setSize] = useState<string | null>(null);
  const [traits, setTraits] = useState<ArtworkTrait[]>([
    {
      trait_type: 'artist_name',
      value: '',
    },
    {
      trait_type: 'medium',
      value: '',
    },
    {
      trait_type: 'year',
      value: '',
    },
    {
      trait_type: 'size',
      value: '',
    },

    {
      trait_type: 'background_color',
      value: '#ffffff',
      display_type: ArtworkTraitDisplayType.Color,
    },
  ]);

  const isArtworkRequiredFieldsFilled = Boolean(
    typeof traits.find((t) => t.trait_type === 'artist_name')?.value ===
      'string' &&
      (
        traits.find((t) => t.trait_type === 'artist_name')?.value as string
      ).trim() !== '' &&
      typeof traits.find((t) => t.trait_type === 'background_color')?.value ===
        'string' &&
      (
        traits.find((t) => t.trait_type === 'background_color')?.value as string
      ).trim() !== '' &&
      typeof traits.find((t) => t.trait_type === 'size')?.value === 'string' &&
      (traits.find((t) => t.trait_type === 'size')?.value as string).trim() !==
        '' &&
      typeof traits.find((t) => t.trait_type === 'medium')?.value ===
        'string' &&
      (
        traits.find((t) => t.trait_type === 'medium')?.value as string
      ).trim() !== '',
  );
  const isAllFieldsFilled = Boolean(
    title &&
      title.trim() !== '' &&
      content &&
      content.trim() !== '' &&
      (postType !== PostType.Artwork ? true : isArtworkRequiredFieldsFilled),
  );
  const resetState = useCallback(() => {
    setExpand(false);
    setFeed(null);
    setContent('');
    setTitle('');
    setImage(null);
    setStatus(Status.Idle);
    setIsModified(false);
  }, []);

  const toggleExpand = useCallback(() => {
    setExpand((prev) => !prev);
    setClose(true);
  }, []);

  const updateTitle = (newTitle: string) => {
    setTitle(newTitle);
    setIsModified(true);
  };

  const updateContent = (newContent: string) => {
    setContent(newContent);
    setIsModified(true);
  };

  const updatePostType = (type: PostType) => {
    setPostType(type);
    setIsModified(true);
  };

  const { mutateAsync: handleUpdateWithTitleAndContent } =
    useUpdateDraftMutation();
  const { mutateAsync: handleUpdateImage } = useUpdateDraftImageMutation();
  const { mutateAsync: publishDraft } = usePublishDraftMutation();

  const updateImage = async (image: string | null) => {
    if (!image) {
      return;
    }

    const mime = image.match(/^data:([^;]+);base64,/);
    if (mime && mime[1]) {
      const res = await apiFetch<AssetPresignedUris>(
        `${config.api_url}${ratelApi.assets.getPresignedUrl(parseFileType(mime[1]))}`,
        {
          method: 'GET',
        },
      );
      if (
        res.data &&
        res.data.presigned_uris?.length > 0 &&
        res.data.uris?.length > 0
      ) {
        const blob = await dataUrlToBlob(image);
        await fetch(res.data.presigned_uris[0], {
          method: 'PUT',
          headers: {
            'Content-Type': mime[1],
          },
          body: blob,
        });
        const imageUrl = res.data.uris[0];

        await handleUpdateImage({ postPk: feed!.pk, image: imageUrl });

        setImage(imageUrl);
      }
    }
  };

  const updateTrait = (
    trait_type: string,
    value: string,
    display_type: ArtworkTraitDisplayType = ArtworkTraitDisplayType.String,
  ) => {
    setTraits((prevTraits) => {
      const traitIndex = prevTraits.findIndex(
        (t) => t.trait_type === trait_type,
      );
      if (traitIndex !== -1) {
        const updatedTraits = [...prevTraits];
        updatedTraits[traitIndex] = {
          ...updatedTraits[traitIndex],
          value,
          display_type: display_type ?? updatedTraits[traitIndex].display_type,
        };
        return updatedTraits;
      } else {
        return [...prevTraits, { trait_type, value, display_type }];
      }
    });
    setIsModified(true);
  };

  const openPostEditorPopup = async (id: string) => {
    if (status === Status.Loading) {
      return;
    }
    resetState();
    setStatus(Status.Loading);
    try {
      const { post: draft, artwork_metadata } = await getPost(id);
      setFeed(draft);
      setTitle(draft.title || '');
      if (draft.urls.length > 0) {
        setImage(draft.urls[0]);
      }
      setContent(draft.html_contents || '');

      setPostType(
        draft.post_type === PT.Artwork ? PostType.Artwork : PostType.General,
      );

      if (draft.post_type === PT.Artwork && artwork_metadata) {
        setTraits(artwork_metadata.traits || []);
      }
      setExpand(true);
    } catch (e) {
      console.error(e);
      throw new Error('Failed to load draft');
    } finally {
      setStatus(Status.Idle);
      setClose(false);
    }
  };

  const autoSaveDraft = useCallback(async () => {
    if (
      status === Status.Saving ||
      isModified === false ||
      content.length < 50
    ) {
      return;
    }

    setStatus(Status.Saving);

    try {
      await handleUpdateWithTitleAndContent({
        postPk: feed!.pk,
        title,
        content,
      });

      setIsModified(false);
    } catch (error) {
      console.error(error);
      throw new Error(`Failed to auto save draft ${error}`);
    } finally {
      setStatus(Status.Idle);
    }
  }, [
    feed,
    content,
    title,
    status,
    isModified,
    handleUpdateWithTitleAndContent,
  ]);

  useEffect(() => {
    const timeoutId = setInterval(async () => {
      await autoSaveDraft();
    }, AUTO_SAVE_DELAY);
    return () => clearInterval(timeoutId);
  }, [autoSaveDraft]);

  useEffect(() => {
    if (!expand) {
      resetState();
    }
  }, [expand, resetState]);

  useEffect(() => {
    resetState();
  }, [pathname, resetState]);

  const handleUpdate = useCallback(async () => {
    if (status !== Status.Idle || !isAllFieldsFilled) {
      return;
    }
    setStatus(Status.Publishing);

    try {
      if (checkString(title) || checkString(content || '')) {
        throw new Error('Please remove the test keyword');
      }

      await publishDraft({
        postPk: feed!.pk,
        title,
        content,
      });

      router.push(route.threadByFeedId(feed!.pk));
      resetState();
    } catch {
      throw new Error('Failed to publish draft');
    }
  }, [
    content,
    feed,
    isAllFieldsFilled,
    resetState,
    router,
    status,
    title,
    publishDraft,
  ]);

  const contextValue: PostEditorContextType = {
    openPostEditorPopup,

    expand,
    toggleExpand,
    title,
    updateTitle,
    content,
    updateContent,
    image,
    updateImage,
    postType,
    updatePostType,
    traits,
    updateTrait,
    handleUpdate,
    close,
    setClose,
    isSubmitDisabled: !isAllFieldsFilled,
    status,
  };

  return (
    <PostDraftContext.Provider value={contextValue}>
      {children}
    </PostDraftContext.Provider>
  );
}

export const usePostEditorContext = () => {
  const context = useContext(PostDraftContext);
  return context;
};
