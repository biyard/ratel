import SpaceArtworkTradeResponse from '../dto/space-artwork-trade-response';

export default class SpaceArtworkTrade {
  pk: string;
  sk: string;
  createdAt: number;
  nftTokenId: number;
  fromAddress: string;
  toAddress: string;
  transactionHash: string;
  tradeType: TradeType;

  constructor(data: SpaceArtworkTradeResponse) {
    this.pk = data.pk;
    this.sk = data.sk;
    this.createdAt = data.created_at;
    this.nftTokenId = data.nft_token_id;
    this.fromAddress = data.from_address;
    this.toAddress = data.to_address;
    this.transactionHash = data.transaction_hash;
    this.tradeType = data.trade_type;
  }
}

export enum TradeType {
  Mint = 'Mint',
  Transfer = 'Transfer',
}
