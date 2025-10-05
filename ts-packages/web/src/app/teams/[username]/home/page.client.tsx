'use client';
import FeedEndMessage from '@/app/(social)/_components/feed-end-message';
import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import useInfiniteFeeds from '@/hooks/feeds/use-feeds-infinite-query';
import { useObserver } from '@/hooks/use-observer';
import React, { useCallback } from 'react';

export default function TeamHome() {
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
  const flattedPosts = data?.pages.flatMap((page) => page.items) ?? [];

  return (
    <div className="flex-1 flex max-mobile:px-[10px]">
      <Col className="flex-1">
        {flattedPosts.map((post) => (
          <FeedCard key={`feed-${post.pk}`} post={post} />
        ))}

        <div ref={observerRef} />
        {!hasNextPage && <FeedEndMessage />}
      </Col>
    </div>
  );
}
