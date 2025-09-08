import { Feed, FeedStatus, FeedType, UrlType } from '@/lib/api/models/feeds';
import { checkString } from '@/lib/string-filter-utils';
import { useRouter } from 'next/navigation';
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
export interface PostDraftContextType {
  openCreatePostPopup: () => void;
  openCreatePostPopupWithDraft: (id: number) => Promise<void>;

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

export const PostDraftContext = createContext<PostDraftContextType | undefined>(
  undefined,
);
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
export function PostDraftProvider({ children }: { children: React.ReactNode }) {
  const { data: user } = useUserInfo();
  const { selectedTeam } = useTeamContext();
  /*
    If Team is selected, use `team_id` as targetId
    Otherwise, use `user_id` as targetId
    if Not Logged in, use `0` as targetId
  */
  const targetId = selectedTeam?.id || user?.id || 0;

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
  console.log('isAllFieldsFilled', isAllFieldsFilled);
  const resetState = useCallback(() => {
    setExpand(false);
    setDraftId(null);
    setTitle('');
    setImage(null);
    setStatus(Status.Idle);
    setIsModified(false);
  }, []);

  const toggleExpand = useCallback(() => {
    setExpand((prev) => !prev);
  }, []);

  const updateTitle = (newTitle: string) => {
    if (newTitle && newTitle.trim() !== '') {
      setTitle(newTitle);
      setIsModified(true);
    }
  };

  const updateContent = (newContent: string | null) => {
    if (newContent && newContent.trim() !== '') {
      setContent(newContent);
      setIsModified(true);
    } else if (newContent === null) {
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
    } else if (newImage && newImage.trim() !== '') {
      setImage(newImage);
      setIsModified(true);
    }
  };

  const updateBackgroundColor = (newBackgroundColor: string | null) => {
    if (newBackgroundColor && newBackgroundColor.trim() !== '') {
      setBackgroundColor(newBackgroundColor);
      setIsModified(true);
    }
  };
  const updateArtistName = (newArtistName: string | null) => {
    if (newArtistName && newArtistName.trim() !== '') {
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

  const openCreatePostPopup = async () => {
    resetState();
    setExpand(true);
  };

  const openCreatePostPopupWithDraft = async (id: number) => {
    if (status === Status.Loading) {
      return;
    }
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
    let draft_id: number;
    if (!draftId) {
      draft_id = await createDraft(targetId);
      setDraftId(draft_id);
    } else {
      draft_id = draftId;
    }
    await updateDraft(draft_id, {
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

  const handlePublishDraft = useCallback(async () => {
    console.log(
      'handlePublishDraft called',
      status,
      draftId,
      isAllFieldsFilled,
    );
    if (status !== Status.Idle || !draftId || !isAllFieldsFilled) {
      return;
    }
    setStatus(Status.Publishing);

    try {
      if (checkString(title) || checkString(content || '')) {
        throw new Error('Please remove the test keyword');
      }
      await handleUpdateDraft();
      await publishDraft(draftId);
      invalidatePostQuery(targetId);
      router.push(route.threadByFeedId(draftId));
      resetState();
    } catch {
      throw new Error('Failed to publish draft');
    } finally {
      setStatus(Status.Idle);
    }
  }, [
    content,
    draftId,
    handleUpdateDraft,
    isAllFieldsFilled,
    resetState,
    router,
    status,
    targetId,
    title,
  ]);

  const contextValue: PostDraftContextType = {
    openCreatePostPopup,
    openCreatePostPopupWithDraft,

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

export const usePostDraft = () => {
  const context = useContext(PostDraftContext);
  if (context === undefined) {
    throw new Error('usePostDraft must be used within a PostDraftProvider');
  }
  return context;
};

//   const loadDraft = useCallback(
//     async (id: number) => {
//       setStatus(DraftStatus.Loading);

//       try {
//         const draft: Feed = await get(ratelApi.feeds.getFeedsByFeedId(id));
//         const draftTitle = draft.title || '';
//         const draftContent = draft.html_contents || '';
//         const draftImage =
//           draft.url && draft.url_type === UrlType.Image ? draft.url : null;
//         const isPublished = draft.status === FeedStatus.Published;

//         // Reset content first to clear the editor
//         setContent(null);

//         // React 18+ automatically batches these state updates
//         setDraftId(draft.id);
//         setTitle(draftTitle);
//         setImage(draftImage);

//         setContent(draftContent);
//         setExpand(true);
//       } catch (error: unknown) {
//         logger.error('Failed to load draft:', error);
//         setStatus('error');
//       } finally {
//         setStatus('idle');
//       }
//     },
//     [get],
//   );

//   const saveDraft = useCallback(
//     async (
//       currentTitle: string,
//       currentContent: string,
//       currentImage: string | null,
//     ) => {
//       if (status === 'saving' || status === 'creating' || !user) return;

//       if (!currentTitle.trim()) return;

//       const lastSaved = lastSavedRef.current;
//       const hasChanges =
//         currentTitle !== lastSaved.title ||
//         currentContent !== lastSaved.content ||
//         currentImage !== lastSaved.image;

//       if (!hasChanges) return;

//       const isCreating = !draftId;
//       setStatus(isCreating ? 'creating' : 'saving');

//       try {
//         let currentDraftId = draftId;

//         if (checkString(currentTitle) || checkString(currentContent)) {
//           showErrorToast('Please remove the test keyword');
//           return;
//         }

//         if (isCreating) {
//           const data: Feed = await post(
//             ratelApi.feeds.createDraft(),
//             createDraftRequest(FeedType.Post, user.id),
//           );
//           currentDraftId = data.id;
//           setDraftId(currentDraftId);
//         }

//         if (currentDraftId) {
//           let url = '';
//           let url_type = UrlType.None;
//           if (currentImage) {
//             url = currentImage;
//             url_type = UrlType.Image;
//           }

//           if (isPublishedPost) {
//             await post(
//               ratelApi.feeds.editPost(currentDraftId),
//               editPostRequest(
//                 currentContent,
//                 1,
//                 currentTitle,
//                 0,
//                 [],
//                 url,
//                 url_type,
//               ),
//             );
//           } else {
//             await post(
//               ratelApi.feeds.updateDraft(currentDraftId),
//               updateDraftRequest(
//                 currentContent,
//                 1,
//                 currentTitle,
//                 0,
//                 [],
//                 url,
//                 url_type,
//                 postType === PostType.Artwork
//                   ? FeedType.Artwork
//                   : FeedType.Post,
//               ),
//             );
//           }

//           lastSavedRef.current = {
//             title: currentTitle,
//             content: currentContent,
//             image: currentImage,
//           };
//         }

//         refetchDrafts();
//       } catch (error: unknown) {
//         logger.error('Failed to save draft:', error);
//         setStatus('error');
//       } finally {
//         setStatus('idle');
//       }
//     },
//     [draftId, user, post, refetchDrafts, status, isPublishedPost],
//   );

//   useEffect(() => {
//     // Only auto-save for drafts, not published posts
//     if (!title.trim() && !content?.trim()) return;
//     if (status !== 'idle' || isPublishedPost) return;

//     const lastSaved = lastSavedRef.current;
//     const hasChanges =
//       title !== lastSaved.title ||
//       content !== lastSaved.content ||
//       image !== lastSaved.image;

//     if (!hasChanges || content === null) return;

//     const AUTO_SAVE_DELAY = 1500; // ms
//     const timeoutId = setTimeout(() => {
//       saveDraft(title, content, image);
//     }, AUTO_SAVE_DELAY);

//     return () => clearTimeout(timeoutId);
//   }, [title, content, image, status, saveDraft, isPublishedPost]);

//   const publishPost = useCallback(async () => {
//     if (checkString(title) || checkString(content ?? '')) {
//       showErrorToast('Please remove the test keyword');
//       return;
//     }

//     if (!draftId || !title.trim() || status !== 'idle' || content == null) {
//       return;
//     }

//     setStatus('publishing');

//     try {
//       await saveDraft(title, content, image);

//       await post(ratelApi.feeds.publishDraft(draftId), {
//         publish: {},
//       });

//       router.push(route.threadByFeedId(draftId));
//       resetState();
//       setExpand(false);
//       refetchDrafts();
//     } catch (error: unknown) {
//       logger.error('Failed to publish post:', error);
//       setStatus('error');
//     } finally {
//       setStatus('idle');
//     }
//   }, [
//     draftId,
//     title,
//     content,
//     image,
//     status,
//     saveDraft,
//     post,
//     resetState,
//     refetchDrafts,
//     router,
//   ]);

//   const savePost = useCallback(async () => {
//     if (checkString(title) || checkString(content ?? '')) {
//       showErrorToast('Please remove the test keyword');
//       return;
//     }

//     if (!draftId || !title.trim() || status !== 'idle' || content == null) {
//       return;
//     }

//     try {
//       await saveDraft(title, content, image);

//       // Invalidate the specific feed query to refetch updated data
//       queryClient.invalidateQueries({
//         queryKey: [QK_GET_FEED_BY_FEED_ID, draftId],
//       });

//       // Close the editor and reset state
//       setExpand(false);
//       resetState();
//     } catch (error: unknown) {
//       logger.error('Failed to save post changes:', error);
//     }
//   }, [
//     draftId,
//     title,
//     content,
//     image,
//     status,
//     saveDraft,
//     queryClient,
//     setExpand,
//     resetState,
//   ]);
