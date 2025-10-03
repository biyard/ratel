'use client';
import FeedEndMessage from '@/app/(social)/_components/feed-end-message';
import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import useInfiniteFeeds from '@/hooks/feeds/use-feeds-infinite-query';
import { useObserver } from '@/hooks/use-observer';
import { UserType } from '@/lib/api/models/user';
import React, { useCallback } from 'react';

interface TeamHomeProps {
  teamId: number;
}

export default function TeamHome({ teamId }: TeamHomeProps) {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteFeeds();

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
  const flattedPosts = data?.pages.flatMap((page) => page.posts) ?? [];

  return (
    <div className="flex-1 flex max-mobile:px-[10px]">
      <Col className="flex-1">
        {flattedPosts.map((post) => (
          <FeedCard
            key={`feed-${post.id}`}
            contents={post.html_contents || ''}
            author_profile_url={post?.author?.[0]?.profile_url || ''}
            author_name={post?.author?.[0]?.nickname || ''}
            author_type={post?.author?.[0]?.user_type || UserType.Anonymous}
            author_id={post?.author?.[0]?.id || 0}
            user_id={post.user_id || 0}
            id={post.id}
            url={post.url || ''}
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
    </div>
  );
}
