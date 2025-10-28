import { QK_GET_NEWS_BY_NEWS_ID } from '@/constants';
import { ratelApi } from '@/lib/api/ratel_api';
import { apiFetch, type FetchResponse } from '@/lib/api/apiFetch';
import { config } from '@/config';
import {
  useSuspenseQuery,
  type UseSuspenseQueryResult,
} from '@tanstack/react-query';
import type { NewsDetail } from '@/lib/api/models/news';

export function getKey(id: number): [string, number] {
  return [QK_GET_NEWS_BY_NEWS_ID, id];
}

export function useNewsByID(
  id: number,
): UseSuspenseQueryResult<NewsDetail | null> {
  const query = useSuspenseQuery({
    queryKey: getKey(id),
    queryFn: async () => {
      const { data } = await requestNewsByID(id);
      return data;
    },
    refetchOnWindowFocus: false,
  });

  return query;
}

export function requestNewsByID(
  id: number,
): Promise<FetchResponse<NewsDetail | null>> {
  return apiFetch<NewsDetail | null>(
    `${config.api_url}${ratelApi.news.getNewsDetails(id)}`,
    {
      ignoreError: true,
    },
  );
}
