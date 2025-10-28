import { QK_GET_NEWS_BY_NEWS_ID } from '@/constants';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  useSuspenseQuery,
  type UseSuspenseQueryResult,
} from '@tanstack/react-query';
import type { NewsDetailItem } from '@/lib/api/models/news';
import { useApiCall } from '@/lib/api/use-send';

export function useNews(): UseSuspenseQueryResult<NewsDetailItem> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_NEWS_BY_NEWS_ID],
    queryFn: () => get(ratelApi.news.getNews(1, 3)),
    refetchOnWindowFocus: false,
  });

  return query;
}
