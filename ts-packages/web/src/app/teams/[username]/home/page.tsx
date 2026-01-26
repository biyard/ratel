import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';
import useTeamInfiniteFeeds from '@/hooks/feeds/use-team-feeds-infinite-query';
import { useObserver } from '@/hooks/use-observer';
import { useCallback } from 'react';
import { useParams } from 'react-router';
import { FeedEndMessage } from '@/features/drafts/components/list-drafts';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';

export default function TeamHomePage() {
  const { username } = useParams<{ username: string }>();
  const { data: team } = useSuspenseFindTeam(username);

  const teamPk = team?.pk || '';

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useTeamInfiniteFeeds(teamPk);

  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);

  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  const flattedPosts = data?.pages.flatMap((page) => page.items) ?? [];

  if (flattedPosts.length === 0) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        No posts available
      </div>
    );
  }

  return (
    <div className="flex-1 flex max-mobile:px-[10px]">
      <Col className="flex-1">
        {flattedPosts.map((post) => (
          <FeedCard key={`feed-${post.pk}`} post={post} />
        ))}

        <div ref={observerRef} />
        {!hasNextPage && (
          //FIXME: use i18n
          <FeedEndMessage msg="You have reached the end of your feed." />
        )}
      </Col>
    </div>
  );
}
