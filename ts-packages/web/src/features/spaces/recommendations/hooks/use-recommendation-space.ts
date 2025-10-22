import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { spaceKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { SpaceRecommendationResponse } from '../types/recommendation-response';

export function getOption(spacePk: string) {
  return {
    queryKey: spaceKeys.recommendation(spacePk),
    queryFn: async () => {
      const recommendation = await call(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/recommendations`,
      );
      return new SpaceRecommendationResponse(recommendation);
    },
    refetchOnWindowFocus: false,
  };
}

export default function useRecommendationSpace(
  spacePk: string,
): UseSuspenseQueryResult<SpaceRecommendationResponse> {
  const query = useSuspenseQuery(getOption(spacePk));
  return query;
}
