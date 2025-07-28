'use client';
import { useEffect } from 'react';
import { useInView } from 'react-intersection-observer';

import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import News from './_components/News';
import BlackBox from './_components/black-box';
import CreatePostButton from './_components/create-post-button';
import { usePostInfinite } from './_hooks/use-posts';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { usePromotion } from './_hooks/use_promotion';
import { useFeedByID } from './_hooks/use-feed';

import { checkString } from '@/lib/string-filter-utils';
import { UserType } from '@/lib/api/models/user';

import FeedEmptyState from './_components/feed-empty-state';
import FeedEndMessage from './_components/feed-end-message';
import PromotionCard from './_components/promotion-card';
import Loading from '@/app/loading';
import Suggestions from './_components/suggestions';
import { Space } from '@/lib/api/models/spaces';

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

export default function Home() {
  const { data: promotion } = usePromotion();
  const { data: feed } = useFeedByID(promotion.feed_id);
  const { data: userInfo } = useSuspenseUserInfo();
  const userId = userInfo?.id || 0;

  const { ref, inView } = useInView({ threshold: 0.5 });

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } =
    usePostInfinite(SIZE);

  const posts = data?.pages.flatMap((page) => page.items) || [];

  const filteredFeeds = posts
    .map((item) => ({
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
      spaces: item.spaces ?? [],
    }))
    .filter((d) => {
      const hasInvalidString =
        checkString(d.title) ||
        checkString(d.contents) ||
        checkString(d.author_name);
      return !hasInvalidString;
    });

  useEffect(() => {
    if (inView && hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [inView, hasNextPage, isFetchingNextPage, fetchNextPage]);

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

            {(isLoading || isFetchingNextPage) && (
              <div className="flex justify-center my-4">
                <Loading />
              </div>
            )}

            {hasNextPage && !isLoading && !isFetchingNextPage && (
              <div ref={ref} className="h-10" />
            )}

            {!hasNextPage && <FeedEndMessage />}
          </Col>
        ) : (
          <FeedEmptyState />
        )}
      </Col>

      <div className="tablet:hidden fixed bottom-4 right-4 z-50">
        <CreatePostButton />
      </div>

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
