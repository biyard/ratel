import { call } from './call';

export function getTopPromotion(): Promise<TopPromotionResponse> {
  return call('GET', '/v3/promotions/top');
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
