'use client';

import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';

import { UserType } from '@/lib/api/models/user';

import { type Space } from '@/lib/api/models/spaces';
import FeedEndMessage from '../feed-end-message';
import CreatePostButton from '../create-post-button';
import PromotionCard from '../promotion-card';
import Suggestions from '../suggestions';
import DisableBorderCard from '../disable-border-card';
import { usePostEditorContext } from '../post-editor/provider';
import { useObserver } from '@/hooks/use-observer';
import { useCallback } from 'react';
import { type TopPromotionResponse } from '@/lib/api/ratel/promotions.v3';
import useInfinitePosts from '../../../../features/posts/hooks/use-posts';

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
}: {
  promotion?: TopPromotionResponse;
}) {
  const postEditorContext = usePostEditorContext();
  const close = postEditorContext?.close;

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } =
    useInfinitePosts();

  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);
  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  if (isLoading) {
    return (
      <div className="flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]">
        Loading...
      </div>
    );
  }

  if (!data || data.pages.length === 0) {
    return (
      <div className="flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]">
        No posts available
      </div>
    );
  }
  const flattedPosts = data?.pages.flatMap((page) => page.items) ?? [];
  return (
    <div className="flex relative flex-1">
      <Col className="flex flex-1 max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedPosts.map((post) => (
            <FeedCard key={`feed-${post.pk}`} post={post} />
          ))}

          <div ref={observerRef} />
          {!hasNextPage && <FeedEndMessage />}
        </Col>
      </Col>

      {close && (
        <div className="hidden max-tablet:!block fixed bottom-4 right-4 z-50">
          <CreatePostButton />
        </div>
      )}

      <aside className="w-70 pl-4 max-tablet:!hidden" aria-label="Sidebar">
        <CreatePostButton />

        {promotion && (
          <DisableBorderCard>
            <PromotionCard promotion={promotion} />
          </DisableBorderCard>
        )}

        {/* TODO: implement with v3
            <div className="mt-[10px]">
          <News />
        </div> */}
        <div className="mt-[10px]">
          <Suggestions />
        </div>
      </aside>
    </div>
  );
}
