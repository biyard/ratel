import { useInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { ListPostResponse } from '../dto/list-post-response';
import { call } from '@/lib/api/ratel/call';
import { FeedStatus } from '../types/post';

export async function listPosts(
  bookmark?: string,
  authorPk?: string,
  status?: FeedStatus,
): Promise<ListPostResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  if (authorPk) {
    params.append('author_pk', authorPk);
  }
  if (status !== undefined) {
    params.append('status', status.toString());
  }

  const queryString = params.toString();
  const path = `/v3/posts${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function getOptions() {
  return {
    queryKey: feedKeys.list({ status: FeedStatus.Published }),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => listPosts(pageParam),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfinitePosts() {
  return useInfiniteQuery(getOptions());
}
