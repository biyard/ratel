import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { call } from './call';
import { QK_RATEL_TOP_PROMOTION } from './constants';

export function getTopPromotion(): Promise<TopPromotionResponse> {
  return call('GET', '/v3/promotions/top');
}

export function useTopPromotion(): UseSuspenseQueryResult<TopPromotionResponse> {
  return useSuspenseQuery({
    queryKey: [QK_RATEL_TOP_PROMOTION],
    queryFn: async () => {
      try {
        return await getTopPromotion();
      } catch {
        return null;
      }
    },
  });
}

export type TopPromotionResponse = {
  id: number;
  created_at: number;
  updated_at: number;

  name: string;
  image_url: string;

  feed_id: number;
  space_id?: number;
  space_type?: number;
};
