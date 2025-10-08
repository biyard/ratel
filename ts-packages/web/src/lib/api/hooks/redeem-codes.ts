import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { RedeemCode } from '../models/redeem-code';
import { useApiCall } from '../use-send';
import { QK_GET_REDEEM_CODE } from '@/constants';
import { ratelApi } from '../ratel_api';

export function useRedeemCode(
  meta_id: number,
): UseSuspenseQueryResult<RedeemCode> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_REDEEM_CODE, meta_id],
    queryFn: () => get(ratelApi.spaces.getSpaceRedeemCodes(meta_id)),
    refetchOnWindowFocus: false,
  });

  return query;
}
