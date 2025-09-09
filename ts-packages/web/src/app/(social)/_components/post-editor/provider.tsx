import { Feed, FeedStatus, FeedType, UrlType } from '@/lib/api/models/feeds';
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
import { useUserInfo } from '../../_hooks/user';
import { useTeamContext } from '@/lib/contexts/team-context';
import { useDraftMutations } from '@/hooks/feeds/use-create-feed-mutation';
import { UpdatePostRequest } from '@/lib/api/models/feeds/update-post';

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
  openPostEditorPopup: (postId?: number) => Promise<void>;
  // openPostEditorPopupWithState: (id: number) => Promise<void>;

  expand: boolean;
  toggleExpand: () => void;
  postType: PostType;
  updatePostType: (type: PostType) => void;

  title: string;
  updateTitle: (title: string) => void;
  content: string | null;
  updateContent: (content: string | null) => void;
  image: string | null;
  updateImage: (image: string | null) => void;

  artistName: string | null;
  updateArtistName: (artistName: string | null) => void;
  backgroundColor: string;
  updateBackgroundColor: (backgroundColor: string) => void;
  size: string | null;
  updateSize: (size: string | null) => void;

  handleUpdate: () => Promise<void>;

  isSubmitDisabled: boolean;
  status: Status;
}

export const PostDraftContext = createContext<
  PostEditorContextType | undefined
>(undefined);

async function loadDraft(id: number): Promise<Feed> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.getFeed(id)}`,
  );
  if (!res.data) {
    throw new Error('Draft not found');
  }
  return res.data;
}

export function PostEditorProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data: user } = useUserInfo();
  const { selectedTeam } = useTeamContext();
  const { createDraft, updateDraft, publishDraft } = useDraftMutations();
  /*
    If Team is selected, use `team_id` as targetId
    Otherwise, use `user_id` as targetId
    if Not Logged in, use `0` as targetId
  */
  const targetId = selectedTeam?.id || user?.id || 0;

  const pathname = usePathname();

  //Interal State
  const router = useRouter();
  const [expand, setExpand] = useState(false);
  const [status, setStatus] = useState<Status>(Status.Idle);
  const [feed, setFeed] = useState<Feed | null>(null);
  const [postType, setPostType] = useState<PostType>(PostType.General);
  const [isModified, setIsModified] = useState(false);

  //State
  const [title, setTitle] = useState('');
  const [content, setContent] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(null);
  const [artistName, setArtistName] = useState<string | null>(null);
  const [backgroundColor, setBackgroundColor] = useState<string>('#ffffff');
  const [size, setSize] = useState<string | null>(null);

  const isAllFieldsFilled = Boolean(
    title &&
      title.trim() !== '' &&
      content &&
      content.trim() !== '' &&
      (postType !== PostType.Artwork
        ? true
        : artistName && backgroundColor && size && image),
  );
  const resetState = useCallback(() => {
    setExpand(false);
    setFeed(null);
    setContent(null);
    setTitle('');
    setImage(null);
    setStatus(Status.Idle);
    setIsModified(false);
  }, []);

  const toggleExpand = useCallback(() => {
    setExpand((prev) => !prev);
  }, []);

  const updateTitle = (newTitle: string) => {
    setTitle(newTitle);
    setIsModified(true);
  };

  const updateContent = (newContent: string | null) => {
    setContent(newContent);
    setIsModified(true);
  };

  const updatePostType = (type: PostType) => {
    setPostType(type);
    setIsModified(true);
  };

  const updateImage = (newImage: string | null) => {
    setImage(newImage);
    setIsModified(true);
  };

  const updateBackgroundColor = (newBackgroundColor: string | null) => {
    if (newBackgroundColor) {
      setBackgroundColor(newBackgroundColor);
    }
    setIsModified(true);
  };
  const updateArtistName = (newArtistName: string | null) => {
    setArtistName(newArtistName);
    setIsModified(true);
  };
  const updateSize = (newSize: string | null) => {
    setSize(newSize);
    setIsModified(true);
  };

  const openPostEditorPopup = async (id?: number) => {
    if (!id) {
      resetState();
      setExpand(true);
      return;
    }
    if (status === Status.Loading) {
      return;
    }
    resetState();
    setStatus(Status.Loading);
    try {
      const draft = await loadDraft(id);
      setFeed(draft);
      setTitle(draft.title || '');
      if (draft.url_type === UrlType.Image && draft.url) {
        setImage(draft.url);
      }
      setContent(draft.html_contents || '');

      setPostType(
        draft.feed_type === FeedType.Artwork
          ? PostType.Artwork
          : PostType.General,
      );
      console.log('DRAFT META', draft.artwork_metadata);

      if (draft.feed_type === FeedType.Artwork && draft.artwork_metadata) {
        setArtistName(draft.artwork_metadata.artist_name || null);
        setBackgroundColor(draft.artwork_metadata.background_color);
        setSize(draft.artwork_metadata.size || null);
      }
      setExpand(true);
    } catch {
      throw new Error('Failed to load draft');
    } finally {
      setStatus(Status.Idle);
    }
  };

  const handleUpdateDraft = useCallback(async () => {
    let id: number;
    if (!feed) {
      const newFeed = await createDraft.mutateAsync(targetId);
      id = newFeed.id;
      setFeed(newFeed);
    } else {
      id = feed.id;
    }

    const req: Partial<UpdatePostRequest> = {
      title,
      html_contents: content || undefined,
      url: image || undefined,
      url_type: image ? UrlType.Image : UrlType.None,
      feed_type:
        postType === PostType.Artwork ? FeedType.Artwork : FeedType.Post,
      artwork_metadata:
        postType === PostType.Artwork
          ? {
              artist_name: artistName || '',
              background_color: backgroundColor || '',
              size: size || '',
            }
          : undefined,
    };

    await updateDraft.mutateAsync({
      postId: id,
      req,
    });
    return id;
  }, [
    feed,
    title,
    content,
    image,
    postType,
    artistName,
    backgroundColor,
    size,
    updateDraft,
    createDraft,
    targetId,
  ]);
  const autoSaveDraft = useCallback(async () => {
    if (status === Status.Saving || isModified === false) {
      return;
    }

    setStatus(Status.Saving);

    try {
      await handleUpdateDraft();
      setIsModified(false);
    } catch (error) {
      console.error(error);
      throw new Error('Failed to auto save draft');
    } finally {
      setStatus(Status.Idle);
    }
  }, [status, isModified, handleUpdateDraft]);

  useEffect(() => {
    const timeoutId = setInterval(async () => {
      await autoSaveDraft();
    }, AUTO_SAVE_DELAY);
    return () => clearInterval(timeoutId);
  }, [autoSaveDraft]);

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
      const finalDraftId = await handleUpdateDraft();
      if (feed?.status !== FeedStatus.Published) {
        await publishDraft.mutateAsync(
          {
            draftId: finalDraftId,
          },
          {
            onSuccess: () => {
              router.push(route.threadByFeedId(finalDraftId));
              resetState();
            },
          },
        );
      }
      resetState();
    } catch {
      throw new Error('Failed to publish draft');
    }
  }, [
    content,
    feed?.status,
    handleUpdateDraft,
    isAllFieldsFilled,
    publishDraft,
    resetState,
    router,
    status,
    title,
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
    artistName,
    updateArtistName,
    backgroundColor,
    updateBackgroundColor,
    size,
    updateSize,

    handleUpdate,
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
  if (context === undefined) {
    throw new Error(
      'usePostEditorContext must be used within a PostEditorProvider',
    );
  }
  return context;
};
