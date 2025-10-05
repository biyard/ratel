'use client';
import React, { useCallback } from 'react';
import { Col } from '@/components/ui/col';
import FeedCard from '@/components/feed-card';
import CreatePostButton from '../_components/create-post-button';
import { useObserver } from '@/hooks/use-observer';
import FeedEndMessage from '../_components/feed-end-message';
import useInfiniteMyPosts from './_hooks/use-my-posts';

export default function MyPostsPage() {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteMyPosts();

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
    <div className="flex-1 flex relative">
      <Col className="flex-1 flex max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedPosts.map((post) => (
            <FeedCard key={`feed-${post.pk}`} post={post} />
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
