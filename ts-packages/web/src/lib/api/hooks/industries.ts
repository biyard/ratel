import { useSuspenseQuery } from '@tanstack/react-query';
import { ratelApi } from '../ratel_api';
import { Industry } from '../models/industry';
import { useApiCall } from '../use-send';

export function useIndustries(): Industry[] {
  const { get } = useApiCall();
  const { data } = useSuspenseQuery<Industry[]>({
    queryKey: ['industries'],
    queryFn: async () => (await get(ratelApi.industries.list())) as Industry[],
    refetchOnWindowFocus: false,
  });
  return data ?? [];
}
