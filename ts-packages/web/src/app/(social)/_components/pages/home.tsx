'use client';
import { useCallback } from 'react';

import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';

import { UserType } from '@/lib/api/models/user';

import { Space } from '@/lib/api/models/spaces';
import FeedEndMessage from '../feed-end-message';
import CreatePostButton from '../create-post-button';
import PromotionCard from '../promotion-card';
import News from '../News';
import Suggestions from '../suggestions';
import { Promotion } from '@/lib/api/models/promotion';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import useInfiniteFeeds from '@/hooks/feeds/use-feeds-infinite-query';
import { useObserver } from '@/hooks/use-observer';
import DisableBorderCard from '../disable-border-card';

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

  spaces: Space[];
}

export default function Home({
  promotion,
  feed,
}: {
  promotion: Promotion | undefined | null;
  feed: Feed | undefined | null;
}) {
  const { data: userInfo } = useSuspenseUserInfo();
  const userId = userInfo?.id || 0;

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteFeeds(0, FeedStatus.Published);
  console.log('data', data);
  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);
  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  if (data.pages.length === 0) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        No drafts available
      </div>
    );
  }
  const flattedPosts = data?.pages.flatMap((page) => page) ?? [];
  return (
    <div className="flex-1 flex relative">
      <Col className="flex-1 flex max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedPosts.map((post) => (
            <FeedCard
              key={`feed-${post.id}`}
              contents={post.html_contents || ''}
              author_profile_url={post?.author?.[0]?.profile_url || ''}
              author_name={post?.author?.[0]?.nickname || ''}
              author_type={post?.author?.[0]?.user_type || UserType.Anonymous}
              author_id={post?.author?.[0]?.id || 0}
              user_id={userId}
              id={post.id}
              industry={post.industry?.[0]?.name || ''}
              title={post.title || ''}
              created_at={post.created_at || 0}
              likes={post.likes || 0}
              is_liked={post.is_liked || false}
              comments={post.comments || 0}
              rewards={post.rewards || 0}
              shares={post.shares || 0}
              onboard={post.onboard || false}
              space_id={post.space?.[0]?.id}
              space_type={post.space?.[0]?.space_type}
              booster_type={post.space?.[0]?.booster_type}
            />
          ))}

          <div ref={observerRef} />
          {!hasNextPage && <FeedEndMessage />}
        </Col>
      </Col>

      <div className="tablet:hidden fixed bottom-4 right-4 z-50">
        <CreatePostButton />
      </div>

      <aside className="w-70 pl-4 max-tablet:!hidden" aria-label="Sidebar">
        <CreatePostButton />

        {promotion && feed && (
          <DisableBorderCard>
            <PromotionCard promotion={promotion} feed={feed} />
          </DisableBorderCard>
        )}

        <div className="mt-[10px]">
          <News />
        </div>
        <div className="mt-[10px]">
          <Suggestions />
        </div>
      </aside>
    </div>
  );
}
