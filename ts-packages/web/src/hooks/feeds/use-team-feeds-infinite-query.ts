import { useInfiniteQuery } from '@tanstack/react-query';
import { feedKeys } from '@/constants';
import { ListPostResponse, listTeamPosts } from '@/lib/api/ratel/posts.v3';
import { FeedStatus } from '@/lib/api/models/feeds';

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
