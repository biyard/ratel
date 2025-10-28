import { feedKeys } from '@/constants';
import { ListPostResponse } from '@/features/posts/dto/list-post-response';
import { call } from '@/lib/api/ratel/call';
import { FeedStatus } from '@/features/posts/types/post';
import { useSuspenseInfiniteQuery } from '@tanstack/react-query';

export async function listTeamDrafts(
  teamPk: string,
  bookmark?: string,
  status?: FeedStatus,
): Promise<ListPostResponse> {
  let path = `/v3/teams/${encodeURIComponent(teamPk)}/posts`;
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }
  if (status !== undefined) {
    const separator = path.includes('?') ? '&' : '?';
    path += `${separator}status=${encodeURIComponent(status.toString())}`;
  }

  return call('GET', path);
}

export function getOptions(teamPk: string) {
  return {
    queryKey: feedKeys.drafts(teamPk),
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListPostResponse> => listTeamDrafts(teamPk, pageParam),
    getNextPageParam: (last: ListPostResponse) => last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteTeamDrafts(teamPk: string) {
  return useSuspenseInfiniteQuery(getOptions(teamPk));
}
