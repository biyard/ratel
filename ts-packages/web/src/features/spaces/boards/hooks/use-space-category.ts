import { useQuery, UseQueryResult } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';
import { SpaceCategory } from '../types/space-category';

export const getSpaceCategory = async (
  spacePk: string,
): Promise<SpaceCategory> => {
  const response = await call(
    'GET',
    `/v3/spaces/${encodeURIComponent(spacePk)}/boards/categories`,
  );
  return new SpaceCategory(response);
};

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.boards_category(spacePk),
    queryFn: async () => {
      const post = await getSpaceCategory(spacePk);

      return post;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useSpaceCategory(
  spacePk: string,
): UseQueryResult<SpaceCategory> {
  const query = useQuery({ ...getOption(spacePk) });
  return query;
}
