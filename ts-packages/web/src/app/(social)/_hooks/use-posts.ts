import { ratelApi } from '@/lib/api/ratel_api';
import { QueryResponse } from '@/lib/api/models/common';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import { useApiCall } from '@/lib/api/use-send';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import {
  QK_GET_FEED_BY_FEED_ID,
  QK_GET_POSTS,
  QK_GET_POSTS_BY_USER_ID,
} from '@/constants';

export function usePost(
  page: number,
  size: number,
): UseSuspenseQueryResult<QueryResponse<Feed>> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_POSTS, page, size],
    queryFn: () => get(ratelApi.feeds.getPosts(page, size)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function usePostByFeedId(feed_id: number): UseSuspenseQueryResult<Feed> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_FEED_BY_FEED_ID, feed_id],
    queryFn: () => get(ratelApi.feeds.getFeedsByFeedId(feed_id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export const postByUserIdQk = (
  user_id: number,
  page: number,
  size: number,
  status: FeedStatus = FeedStatus.Published,
) => [QK_GET_POSTS_BY_USER_ID, user_id, page, size, status];

export function usePostByUserId(
  user_id: number,
  page: number,
  size: number,
  status: FeedStatus = FeedStatus.Published,
  initialData?: QueryResponse<Feed>,
): UseSuspenseQueryResult<QueryResponse<Feed>> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: postByUserIdQk(user_id, page, size, status),
    queryFn: () =>
      get(ratelApi.feeds.getPostsByUserId(user_id, page, size, status)),
    refetchOnWindowFocus: false,
    initialData,
  });

  return query;
}
