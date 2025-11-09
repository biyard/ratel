import { useSuspenseInfiniteQuery } from '@tanstack/react-query';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { listMySpaces, ListMySpacesResponse } from '@/lib/api/ratel/me.v3';

export function getOptions(username: string) {
  return {
    queryKey: ['my-spaces', username],
    queryFn: ({
      pageParam,
    }: {
      pageParam?: string;
    }): Promise<ListMySpacesResponse> => listMySpaces(pageParam),
    getNextPageParam: (last: ListMySpacesResponse) =>
      last.bookmark ?? undefined,
    initialPageParam: undefined as string | undefined,
    refetchOnWindowFocus: false,
  };
}

export default function useInfiniteMySpaces() {
  const { data } = useSuspenseUserInfo();

  const username = data?.username || '';

  return useSuspenseInfiniteQuery(getOptions(username));
}
