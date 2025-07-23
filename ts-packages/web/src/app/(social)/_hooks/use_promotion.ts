import { QK_GET_PROMOTION } from '@/constants';
import { Promotion } from '@/lib/api/models/promotion';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useQuery, UseQueryResult } from '@tanstack/react-query';

export function usePromotion(): UseQueryResult<Promotion> {
  const { get } = useApiCall();

  const query = useQuery({
    queryKey: [QK_GET_PROMOTION],
    queryFn: () => get(ratelApi.promotions.get_promotions()),
    refetchOnWindowFocus: false,
  });

  return query;
}
