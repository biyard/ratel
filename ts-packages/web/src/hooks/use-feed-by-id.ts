import { QK_GET_FEED } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import {
  useMutation,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { Feed } from '@/lib/api/models/feeds';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast } from '@/lib/toast';

export const queryKey = (feed_id: number) => [QK_GET_FEED, feed_id];

export async function getFeedById(
  feed_id: number,
): Promise<FetchResponse<Feed | null>> {
  return apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.getFeedsByFeedId(feed_id)}`,
  );
}

export async function likeFeedById(
  feed_id: number,
): Promise<FetchResponse<Feed | null>> {
  const req = {
    like: {},
  };
  return apiFetch<Feed | null>(
    `${config.api_url}${ratelApi.feeds.getFeedsByFeedId(feed_id)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

export function getOption(feed_id: number) {
  return {
    queryKey: queryKey(feed_id),
    queryFn: async () => {
      const { data } = await getFeedById(feed_id);

      if (!data) {
        throw new Error('Feed not found');
      }
      return data;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useFeedById(
  feed_id: number,
): UseSuspenseQueryResult<Feed> {
  const query = useSuspenseQuery(getOption(feed_id));
  return query;
}

export function useFeedMutation(feed_id: number) {
  const queryClient = getQueryClient();
  const qk = queryKey(feed_id);

  const likeMutation = useMutation({
    mutationFn: async (next: boolean) => {
      const { data } = await apiFetch<Feed | null>(
        `${config.api_url}${ratelApi.feeds.likePost(feed_id)}`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ like: { value: next } }),
        },
      );
      if (!data) {
        throw new Error('Like response did not include data.');
      }
      return next;
    },
    onMutate: async (next) => {
      await queryClient.cancelQueries({ queryKey: qk });

      const previousData = queryClient.getQueryData<Feed>(qk);

      queryClient.setQueryData<Feed>(qk, (old) => {
        if (!old) return old;
        const was = old.is_liked;
        const will = next;
        if (was === will) return old;
        const delta = will ? 1 : -1;
        return {
          ...old,
          likes: Math.max(0, old.likes + delta),
          is_liked: will,
        };
      });

      return { previousData };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: qk });
    },
    onError: (error, next, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(qk, context.previousData);
      }
      showErrorToast(error.message || 'Failed to like feed');
    },
  });

  return { like: likeMutation };
}
