import { useInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { ListPostResponse } from '@/features/posts/dto/list-post-response';
import { call } from '@/lib/api/ratel/call';
import { FeedStatus } from '@/features/posts/types/post';

export async function listTeamPosts(
  teamPk: string,
  bookmark?: string,
  status?: FeedStatus,
): Promise<ListPostResponse> {
  const params = new URLSearchParams();
  if (bookmark) {
    params.append('bookmark', bookmark);
  }
  if (status !== undefined) {
    params.append('status', status.toString());
  }
  const queryString = params.toString();
  const path = `/v3/teams/${encodeURIComponent(teamPk)}/posts${queryString ? `?${queryString}` : ''}`;

  return call('GET', path);
}

export function getTeamFeedsOptions(teamPk: string, status?: FeedStatus) {
  return {
    queryKey: [
      ...feedKeys.lists(),
      { teamPk, status: status || FeedStatus.Published },
    ] as const,
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => {
      if (!teamPk) {
        // Return empty result if teamPk is not provided
        return Promise.resolve({ items: [], bookmark: undefined });
      }
      return listTeamPosts(teamPk, pageParam, status);
    },
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
    enabled: !!teamPk, // Only run query when teamPk is available
  };
}

export default function useTeamInfiniteFeeds(
  teamPk: string,
  status?: FeedStatus,
) {
  return useInfiniteQuery(getTeamFeedsOptions(teamPk, status));
}
