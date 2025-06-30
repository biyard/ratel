import { QK_GET_NETWORK } from '@/constants';
import { NetworkData } from '@/lib/api/models/network';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  UseSuspenseQueryResult,
  useSuspenseQuery,
} from '@tanstack/react-query';

export function useNetwork(): UseSuspenseQueryResult<NetworkData> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_NETWORK],
    queryFn: () => get(ratelApi.networks.getNetworks()),
    refetchOnWindowFocus: false,
  });

  return query;
}
