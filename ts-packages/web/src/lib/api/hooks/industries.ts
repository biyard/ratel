import {} from '@tanstack/react-query';
import { ratelApi } from '../ratel_api';
import { Industry } from '../models/industry';
import { useSuspenseQuery } from '@apollo/client';

export function useIndustries(): Industry[] {
  return useSuspenseQuery(ratelApi.graphql.listIndustries().query)
    .data as Industry[];
}
