/* eslint-disable @typescript-eslint/no-explicit-any */

'use client';
import { useState, useEffect, useCallback } from 'react';
import { useInView } from 'react-intersection-observer';

import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import News from './_components/News';
import BlackBox from './_components/black-box';
import CreatePostButton from './_components/create-post-button';
import { usePost } from './_hooks/use-posts';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { usePromotion } from './_hooks/use_promotion';
import { useFeedByID } from './_hooks/use-feed';

import { checkString } from '@/lib/string-filter-utils';
import { UserType } from '@/lib/api/models/user';
import { showErrorToast } from '@/lib/toast';
import { logger } from '@/lib/logger';

import FeedEmptyState from './_components/feed-empty-state';
import FeedEndMessage from './_components/feed-end-message';
import PromotionCard from './_components/promotion-card';
import Loading from '@/app/loading';
import Suggestions from './_components/suggestions';

const FEED_RESET_TIMEOUT_MS = 10000;
export const SIZE = 10;

export interface Post {
  id: number;
  industry: string;
  title: string;
  contents: string;
  url?: string;
  author_id: number;
  author_profile_url: string;
  author_name: string;
  author_type: UserType;
  space_id?: number;
  space_type?: number;
  likes: number;
  is_liked: boolean;
  comments: number;
  rewards: number;
  shares: number;
  created_at: number;
  onboard: boolean;
}

export default function Home() {
  const { data: promotion } = usePromotion();
  const { data: feed } = useFeedByID(promotion.feed_id);
  const { data: userInfo } = useSuspenseUserInfo();
  const userId = userInfo?.id || 0;

  const [page, setPage] = useState(1);
  const [feeds, setFeeds] = useState<Post[]>([]);
  const [hasMore, setHasMore] = useState(true);
  const [showEndMessage, setShowEndMessage] = useState(false);

  const { ref, inView } = useInView({ threshold: 0.5 });

  const { data: postData, error: postError, isLoading } = usePost(page, SIZE);

  // Processing and deduplication of feed data

  const processFeedData = useCallback((items: any[]): Post[] => {
    if (!items) return [];

    return items.map((item) => ({
      id: item.id,
      industry: item.industry?.[0]?.name || '',
      title: item.title!,
      contents: item.html_contents,
      url: item.url,
      author_id: item.author?.[0]?.id || 0,
      author_profile_url: item.author?.[0]?.profile_url || '',
      author_name: item.author?.[0]?.nickname || '',
      author_type: item.author?.[0]?.user_type || UserType.Anonymous,
      space_id: item.spaces?.[0]?.id || 0,
      space_type: item.spaces?.[0]?.space_type || 0,
      likes: item.likes,
      is_liked: item.is_liked,
      comments: item.comments,
      rewards: item.rewards,
      shares: item.shares,
      created_at: item.created_at,
      onboard: item.onboard ?? false,
    }));
  }, []);

  useEffect(() => {
    if (postError) {
      showErrorToast('Failed to load posts');
      logger.error('Failed to load posts:', postError);
      return;
    }

    if (!postData?.items) return;

    const newFeeds = processFeedData(postData.items);

    if (newFeeds.length === 0) {
      setHasMore(false);
      setShowEndMessage(true);

      const timeout = setTimeout(() => {
        setFeeds([]);
        setPage(1);
        setHasMore(true);
        setShowEndMessage(false);
      }, FEED_RESET_TIMEOUT_MS);

      return () => clearTimeout(timeout);
    }

    setFeeds((prevFeeds) => {
      const uniqueMap = new Map<number, Post>();
      [...prevFeeds, ...newFeeds].forEach((feed) =>
        uniqueMap.set(feed.id, feed),
      );
      return Array.from(uniqueMap.values());
    });
  }, [postData, postError, processFeedData]);

  useEffect(() => {
    if (inView && hasMore && !isLoading) {
      setPage((prev) => prev + 1);
    }
  }, [inView, hasMore, isLoading]);

  const filteredFeeds = feeds.filter(
    (d) =>
      !(
        checkString(d.title) ||
        checkString(d.contents) ||
        checkString(d.author_name)
      ),
  );

  return (
    <div className="flex-1 flex relative">
      <Col className="flex-1 flex max-mobile:px-[10px]">
        {filteredFeeds.length > 0 ? (
          <Col className="flex-1">
            {filteredFeeds.map((props) => (
              <FeedCard
                key={`feed-${props.id}`}
                user_id={userId}
                refetch={() => {}}
                {...props}
              />
            ))}

            {/* Loading state */}
            {isLoading && (
              <div className="flex justify-center my-4">
                <Loading />
              </div>
            )}

            {/* Load more sentinel */}
            {hasMore && !isLoading && <div ref={ref} className="h-10" />}

            {showEndMessage && <FeedEndMessage />}
          </Col>
        ) : (
          <FeedEmptyState />
        )}
      </Col>

      {/* Right Sidebar */}
      <aside className="w-70 pl-4 max-tablet:!hidden" aria-label="Sidebar">
        <CreatePostButton />

        <BlackBox>
          <PromotionCard promotion={promotion} feed={feed} />
        </BlackBox>

        <News />

        <div className="mt-[10px]">
          <Suggestions />
        </div>
      </aside>
    </div>
  );
}
