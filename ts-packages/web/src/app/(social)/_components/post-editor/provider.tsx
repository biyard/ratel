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
import { updateDraftRequest } from '@/lib/api/models/feeds/update-draft-request';
import { route } from '@/route';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import { useUserInfo } from '../../_hooks/user';
import { invalidateQuery as invalidatePostQuery } from '@/hooks/use-post';
import { useTeamContext } from '@/lib/contexts/team-context';

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

  handlePublishDraft: () => Promise<void>;

  isSubmitDisabled: boolean;
  status: Status;
}

export const PostDraftContext = createContext<
  PostEditorContextType | undefined
>(undefined);
export async function createDraft(user_id: number): Promise<number> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.createDraft()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(createDraftRequest(FeedType.Post, user_id)),
    },
  );
  if (!res.data) {
    throw new Error('Failed to create draft');
  }
  return res.data.id;
}

async function loadDraft(id: number): Promise<Feed> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.getFeedsByFeedId(id)}`,
  );
  if (!res.data) {
    throw new Error('Draft not found');
  }
  return res.data;
}

async function updateDraft(
  post_id: number,
  req: Partial<updateDraftRequest>,
): Promise<Feed> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.updateDraft(post_id)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
  if (!res.data) {
    throw new Error('Failed to update draft');
  }
  return res.data;
}

async function publishDraft(id: number): Promise<void> {
  await apiFetch(`${config.api_url}${ratelApi.feeds.publishDraft(id)}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      publish: {},
    }),
  });
}
export function PostEditorProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data: user } = useUserInfo();
  const { selectedTeam } = useTeamContext();
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
  const [draftId, setDraftId] = useState<number | null>(null);
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
    setDraftId(null);
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
    if (newTitle) {
      setTitle(newTitle);
      setIsModified(true);
    }
  };

  const updateContent = (newContent: string | null) => {
    if (newContent && newContent.trim() !== '') {
      setContent(newContent);
      setIsModified(true);
    } else if (newContent === null || newContent.trim() === '') {
      setContent(null);
    }
  };

  const updatePostType = (type: PostType) => {
    setPostType(type);
    setIsModified(true);
  };

  const updateImage = (newImage: string | null) => {
    if (newImage === null) {
      setImage(null);
    } else if (newImage) {
      setImage(newImage);
      setIsModified(true);
    }
  };

  const updateBackgroundColor = (newBackgroundColor: string | null) => {
    if (newBackgroundColor) {
      setBackgroundColor(newBackgroundColor);
      setIsModified(true);
    }
  };
  const updateArtistName = (newArtistName: string | null) => {
    if (newArtistName) {
      setArtistName(newArtistName);
      setIsModified(true);
    }
  };
  const updateSize = (newSize: string | null) => {
    if (newSize && newSize.trim() !== '') {
      setSize(newSize);
      setIsModified(true);
    }
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
      setDraftId(draft.id);
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
    if (!draftId) {
      id = await createDraft(targetId);
      setDraftId(id);
    } else {
      id = draftId;
    }
    await updateDraft(id, {
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
    });
    return id;
  }, [
    draftId,
    title,
    content,
    image,
    postType,
    artistName,
    backgroundColor,
    size,
    targetId,
  ]);
  const autoSaveDraft = useCallback(async () => {
    if (status === Status.Saving || isModified === false) {
      return;
    }

    setStatus(Status.Saving);

    try {
      await handleUpdateDraft();
      invalidatePostQuery(targetId, FeedStatus.Draft);
      setIsModified(false);
    } catch (error) {
      console.error(error);
      throw new Error('Failed to auto save draft');
    } finally {
      setStatus(Status.Idle);
    }
  }, [status, isModified, handleUpdateDraft, targetId]);

  useEffect(() => {
    const timeoutId = setInterval(async () => {
      await autoSaveDraft();
    }, AUTO_SAVE_DELAY);
    return () => clearInterval(timeoutId);
  }, [autoSaveDraft]);

  useEffect(() => {
    resetState();
  }, [pathname, resetState]);

  const handlePublishDraft = useCallback(async () => {
    if (status !== Status.Idle || !isAllFieldsFilled) {
      return;
    }
    setStatus(Status.Publishing);

    try {
      if (checkString(title) || checkString(content || '')) {
        throw new Error('Please remove the test keyword');
      }
      const draftId = await handleUpdateDraft();
      await publishDraft(draftId);
      router.push(route.threadByFeedId(draftId));
      invalidatePostQuery(targetId);
      resetState();
    } catch {
      throw new Error('Failed to publish draft');
    } finally {
      setStatus(Status.Idle);
    }
  }, [
    content,
    handleUpdateDraft,
    isAllFieldsFilled,
    resetState,
    router,
    status,
    targetId,
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

    handlePublishDraft,
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
    throw new Error('usePostDraft must be used within a PostDraftProvider');
  }
  return context;
};
