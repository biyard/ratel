'use client';
import { useEffect } from 'react';
import { useInView } from 'react-intersection-observer';

import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';

import { checkString } from '@/lib/string-filter-utils';
import { UserType } from '@/lib/api/models/user';

import Loading from '@/app/loading';
import { Space } from '@/lib/api/models/spaces';
import { usePostInfinite } from '../../_hooks/use-posts';
import FeedEndMessage from '../feed-end-message';
import FeedEmptyState from '../feed-empty-state';
import CreatePostButton from '../create-post-button';
import PromotionCard from '../promotion-card';
import News from '../News';
import Suggestions from '../suggestions';
import { Promotion } from '@/lib/api/models/promotion';
import { Feed } from '@/lib/api/models/feeds';
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

  const { ref, inView } = useInView({ threshold: 0.5 });

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } =
    usePostInfinite(SIZE);

  const posts = data?.pages.flatMap((page) => page.items) || [];

  const filteredFeeds = posts
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    .filter((item) => Number((item as any).feed_type) !== 2)
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
      booster_type: item.spaces?.[0]?.booster_type,
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
