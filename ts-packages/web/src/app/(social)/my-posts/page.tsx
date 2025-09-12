'use client';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import React, { useCallback } from 'react';
import { Col } from '@/components/ui/col';
import FeedCard from '@/components/feed-card';
import CreatePostButton from '../_components/create-post-button';
import useInfiniteFeeds from '@/hooks/feeds/use-feeds-infinite-query';
import { FeedStatus } from '@/lib/api/models/feeds';
import { useObserver } from '@/hooks/use-observer';
import { UserType } from '@/lib/api/models/user';
import FeedEndMessage from '../_components/feed-end-message';

export default function MyPostsPage() {
  const { data: user } = useSuspenseUserInfo();
  const userId = user.id || 0;

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteFeeds(userId, FeedStatus.Published);

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

      <div
        className={`h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton />
      </div>
    </div>
  );
}
