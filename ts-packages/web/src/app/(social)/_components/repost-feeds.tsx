'use client';
import { createContext, useContext, useState, useCallback } from 'react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { useUserInfo } from '../_hooks/user';
import { Feed, FeedType } from '@/lib/api/models/feeds';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import {
  updateDraftRequest,
  UrlType,
} from '@/lib/api/models/feeds/update-draft-request';
import { useQueryClient } from '@tanstack/react-query';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useRouter } from 'next/navigation';
import { route } from '@/route';

export interface OriginalPost {
  id: number;
  title: string;
  contents: string;
  author_name: string;
  author_profile_url: string;
  author_id: number;
  created_at: number;
  url?: string;
  industry: string;
  reposts?: { user_id: number; id: number }[];
}

export interface RepostContextType {
  isReposting: boolean;
  showRepostModal: boolean;
  showUnrepostModal: boolean;
  originalPost: OriginalPost | null;
  startRepost: (post: OriginalPost) => void;
  cancelRepost: () => void;
  submitRepost: (onRepostClick?: () => void) => Promise<void>;
  isSubmitting: boolean;
  checkIfReposted: (post: OriginalPost) => boolean;
  confirmUnrepost: () => Promise<void>;
}

const RepostContext = createContext<RepostContextType | undefined>(undefined);

export const RepostProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [isReposting, setIsReposting] = useState(false);
  const [showRepostModal, setShowRepostModal] = useState(false);
  const [showUnrepostModal, setShowUnrepostModal] = useState(false);
  const [originalPost, setOriginalPost] = useState<OriginalPost | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [postDraftMap, setPostDraftMap] = useState<Record<number, number>>({});

  const { post, del } = useApiCall();
  const { data: user } = useUserInfo();
  const queryClient = useQueryClient();
  const router = useRouter();

  const checkIfReposted = useCallback(
    (post: OriginalPost): boolean => {
      if (!user || !post.reposts) return false;

      console.log('User Id..', user.id);
      return post.reposts.some((repost) => repost.user_id === user.id);
    },
    [user],
  );

  const startRepost = useCallback(
    (post: OriginalPost) => {
      if (checkIfReposted(post)) {
        setOriginalPost(post);
        setShowUnrepostModal(true);
        return;
      }

      setOriginalPost(post);
      setShowRepostModal(true);
      setIsReposting(true);
    },
    [checkIfReposted],
  );

  const cancelRepost = useCallback(() => {
    setOriginalPost(null);
    setShowRepostModal(false);
    setShowUnrepostModal(false);
    setIsReposting(false);
  }, []);

  const submitRepost = useCallback(
    async (onRepostClick?: () => void) => {
      if (!originalPost || !user) return;

      setIsSubmitting(true);
      try {
        // : Create a draft
        const draftData: Feed = await post(
          ratelApi.feeds.createDraft(),
          createDraftRequest(FeedType.Post, user.id),
        );

        //  Build formatted content
        const repostTitle = `Re: ${originalPost.title}`;
        const repostContent = `
          <div style="border-left: 3px solid #ccc; padding-left: 15px; margin: 10px 0;">
            <h3>${originalPost.title}</h3>
            <p>${originalPost.contents}</p>
            ${
              originalPost.url
                ? `<img src="${originalPost.url}" alt="Uploaded image" style="width: 100%; border-radius: 8px; object-fit: cover;" />`
                : ''
            }
            <small>Originally posted by ${originalPost.author_name}</small>
          </div>
        `;

        // Create the repost entry
        await post(ratelApi.feeds.repost(), {
          repost: {
            parent_id: originalPost.id,
            user_id: user.id,
            html_contents: originalPost.contents,
            quote_feed_id: null,
          },
        });

        //  Update the draft with repost content
        await post(
          ratelApi.feeds.updateDraft(draftData.id),
          updateDraftRequest(
            repostContent,
            1,
            repostTitle,
            originalPost.id,
            [],
            originalPost.url || '',
            originalPost.url ? UrlType.Image : UrlType.None,
          ),
        );

        setPostDraftMap((prev) => ({
          ...prev,
          [originalPost.id]: draftData.id,
        }));

        // : Publish
        await post(ratelApi.feeds.publishDraft(draftData.id), {
          publish: {},
        });

        showSuccessToast('Post reposted successfully!');
        queryClient.invalidateQueries({ queryKey: ['feeds'] });
        router.push(route.threadByFeedId(draftData.id));

        onRepostClick?.(); // Notify parent (FeedCard/page.client) of successful repost
        cancelRepost();
      } catch (error) {
        logger.error('Repost error:', error);
        showErrorToast('Failed to repost. Please try again.');
      } finally {
        setIsSubmitting(false);
      }
    },
    [originalPost, user, post, router, queryClient, cancelRepost],
  );

  const confirmUnrepost = useCallback(async () => {
    if (!originalPost || !user) return;

    setIsSubmitting(true);
    try {
      const repost = originalPost.reposts?.find((r) => r.user_id === user.id);
      if (!repost) throw new Error('Repost not found');

      await del(ratelApi.feeds.unrepost(repost.id));

      const draftId = postDraftMap[originalPost.id];
      if (draftId) {
        await del(ratelApi.feeds.removeDraft(draftId));
        setPostDraftMap((prev) => {
          const map = { ...prev };
          delete map[originalPost.id];
          return map;
        });
      }

      showSuccessToast('Repost and draft removed successfully!');
      queryClient.invalidateQueries({ queryKey: ['feeds'] });
      queryClient.invalidateQueries({ queryKey: ['drafts'] });
      cancelRepost();
    } catch (error) {
      logger.error('Unrepost error:', error);
      showErrorToast('Failed to remove repost. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  }, [originalPost, user, del, postDraftMap, queryClient, cancelRepost]);

  const contextValue = {
    isReposting,
    showRepostModal,
    showUnrepostModal,
    originalPost,
    startRepost,
    cancelRepost,
    submitRepost,
    isSubmitting,
    checkIfReposted,
    confirmUnrepost,
  };

  return (
    <RepostContext.Provider value={contextValue}>
      {children}
    </RepostContext.Provider>
  );
};

export const useRepost = () => {
  const context = useContext(RepostContext);
  if (!context) {
    throw new Error('useRepost must be used within a RepostProvider');
  }
  return context;
};
