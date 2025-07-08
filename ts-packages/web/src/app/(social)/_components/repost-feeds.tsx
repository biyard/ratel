'use client';

import { createContext, useContext, useState, useCallback } from 'react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { useUserInfo } from '../_hooks/user';
import { Feed, FeedType } from '@/lib/api/models/feeds';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import { updateDraftRequest, UrlType } from '@/lib/api/models/feeds/update-draft-request';
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
}

export interface RepostContextType {
  isReposting: boolean;
  showRepostModal: boolean;
  originalPost: OriginalPost | null;
  repostComment: string;
  setRepostComment: (comment: string) => void;
  startRepost: (post: OriginalPost) => void;
  cancelRepost: () => void;
  submitRepost: () => Promise<void>;
  isSubmitting: boolean;
}

const RepostContext = createContext<RepostContextType | undefined>(undefined);

export const RepostProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [isReposting, setIsReposting] = useState(false);
  const [showRepostModal, setShowRepostModal] = useState(false);
  const [originalPost, setOriginalPost] = useState<OriginalPost | null>(null);
  const [repostComment, setRepostComment] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const { post } = useApiCall();
  const { data: user } = useUserInfo();
  const queryClient = useQueryClient();
  const router = useRouter();

  const startRepost = useCallback((post: OriginalPost) => {
    setOriginalPost(post);
    setRepostComment('');
    setShowRepostModal(true);
    setIsReposting(true);
  }, []);

  const cancelRepost = useCallback(() => {
    setOriginalPost(null);
    setRepostComment('');
    setShowRepostModal(false);
    setIsReposting(false);
  }, []);

  const submitRepost = useCallback(async () => {
    if (!originalPost || !user) return;

    setIsSubmitting(true);
    try {
      // Create a new draft for the repost
      const draftData: Feed = await post(
        ratelApi.feeds.createDraft(),
        createDraftRequest(FeedType.Post, user.id)
      );

      // Prepare repost content
      const repostTitle = `Re: ${originalPost.title}`;
      const repostContent = `
        ${repostComment ? `<p>${repostComment}</p><br/>` : ''}
        <div style="border-left: 3px solid #ccc; padding-left: 15px; margin: 10px 0;">
          <h3>${originalPost.title}</h3>
          <p>${originalPost.contents}</p>
          <small>Originally posted by ${originalPost.author_name}</small>
        </div>
      `;

      // Update the draft with repost content
      await post(
        ratelApi.feeds.updateDraft(draftData.id),
        updateDraftRequest(
          repostContent,
          1,
          repostTitle,
          originalPost.id, //original post ID as parent_id or reference
          [],
          originalPost.url || '',
          originalPost.url ? UrlType.Image : UrlType.None
        )
      );

      // Publish the repost
      await post(ratelApi.feeds.publishDraft(draftData.id), {
        publish: {},
      });

      // Update for original post's share count
      // This needs a specific API endpoint for incrementing shares
      // For now, I w'll use the like endpoint pattern but for shares
      try {
        await post(ratelApi.feeds.likePost(originalPost.id), {
          share: { value: true }, // This might need to be adjusted based on our API
        });
      } catch (error) {
        logger.warn('Failed to update share count:', error);
      }

      showSuccessToast('Post reposted successfully!');
      
      // Navigate to the new repost
      router.push(route.threadByFeedId(draftData.id));
      
      // Invalidated relevant queries
      queryClient.invalidateQueries({ queryKey: ['feeds'] });
      
      cancelRepost();
    } catch (error) {
      logger.error('Repost error:', error);
      showErrorToast('Failed to repost. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  }, [originalPost, user, repostComment, post, router, queryClient, cancelRepost]);

  const contextValue = {
    isReposting,
    showRepostModal,
    originalPost,
    repostComment,
    setRepostComment,
    startRepost,
    cancelRepost,
    submitRepost,
    isSubmitting,
  };

  return (
    <RepostContext.Provider value={contextValue}>
      {children}
    </RepostContext.Provider>
  );
};

export const useRepost = () => {
  const context = useContext(RepostContext);
  if (context === undefined) {
    throw new Error('useRepost must be used within a RepostProvider');
  }
  return context;
};
