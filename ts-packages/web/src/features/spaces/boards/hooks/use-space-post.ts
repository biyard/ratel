import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';
import { SpacePostResponse } from '../types/space-post-response';

export const getSpacePost = async (
  spacePk: string,
  postPk: string,
): Promise<SpacePostResponse> => {
  const response = await call(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}`,
  );
  return new SpacePostResponse(response);
};

export function getOption(spacePk: string, postPk: string) {
  return {
    queryKey: spaceKeys.boards_post(spacePk, postPk),
    queryFn: async () => {
      const post = await getSpacePost(spacePk, postPk);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpacePost(
  spacePk: string,
  postPk: string,
): UseQueryResult<SpacePostResponse> {
  const query = useQuery({ ...getOption(spacePk, postPk) });
  return query;
}
