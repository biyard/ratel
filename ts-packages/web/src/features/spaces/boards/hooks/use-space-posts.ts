import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';
import { ListSpacePostsResponse } from '../types/list-space-posts-response';

export const getSpacePosts = async (
  spacePk: string,
): Promise<ListSpacePostsResponse> => {
  const response = await call(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards`,
  );
  return new ListSpacePostsResponse(response);
};

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.boards_posts(spacePk),
    queryFn: async () => {
      const post = await getSpacePosts(spacePk);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpacePosts(
  spacePk: string,
): UseQueryResult<ListSpacePostsResponse> {
  const query = useQuery({ ...getOption(spacePk) });
  return query;
}
