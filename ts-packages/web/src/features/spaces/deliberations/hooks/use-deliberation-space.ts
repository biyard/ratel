import { spaceKeys } from '@/constants';
import {
  DeliberationSpaceResponse,
  getDeliberationSpace,
} from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.detail(spacePk),
    queryFn: async () => {
      const space = await getDeliberationSpace(spacePk);
      return space;
    },
    refetchOnWindowFocus: false,
  };
}

export default function useDeliberationSpace(
  spacePk: string,
): UseSuspenseQueryResult<DeliberationSpaceResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
