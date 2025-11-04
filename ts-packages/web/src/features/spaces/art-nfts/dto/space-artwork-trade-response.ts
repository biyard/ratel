import { ListResponse } from '@/lib/api/ratel/common';
import { TradeType } from '../types/space-artwork-trade';

export default interface SpaceArtworkTradeResponse {
  pk: string;
  sk: string;
  created_at: number;
  nft_token_id: number;
  from_address: string;
  to_address: string;
  transaction_hash: string;
  trade_type: TradeType;
}

export type ListSpaceArtworkTradeResponse =
  ListResponse<SpaceArtworkTradeResponse>;
