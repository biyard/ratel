// hooks/use-team-drafts.ts
import { feedKeys } from '@/constants';
import { ListPostResponse } from '@/features/posts/dto/list-post-response';
import { call } from '@/lib/api/ratel/call';
import { FeedStatus } from '@/features/posts/types/post';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';

export async function listTeamDrafts(
  teamPk: string,
  bookmark?: string,
  status: FeedStatus = FeedStatus.Draft,
): Promise<ListPostResponse> {
  if (!teamPk) {
    return { items: [], bookmark: null };
  }

  let path = `/v3/teams/${encodeURIComponent(teamPk)}/posts`;
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }
  if (status !== undefined) {
    const sep = path.includes('?') ? '&' : '?';
    path += `${sep}status=${encodeURIComponent(status.toString())}`;
  }
  return call('GET', path);
}

export function getOptions(
  teamPk: string,
  status: FeedStatus = FeedStatus.Draft,
) {
  return {
    queryKey: feedKeys.drafts(teamPk),
    queryFn: ({ pageParam }: { pageParam?: string }) =>
      listTeamDrafts(teamPk, pageParam, status),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteTeamDrafts(
  teamPk: string,
  status?: FeedStatus,
) {
  return useSuspenseInfiniteQuery(
    getOptions(teamPk, status ?? FeedStatus.Draft),
  );
}
