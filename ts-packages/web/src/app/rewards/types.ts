// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
// pub struct MyRewardsResponse {
//     // Project Info
//     pub project_name: String,
//     pub token_symbol: String,

//     pub month: String,
//     pub total_points: i64,

//     pub user_points: i64,
//     pub monthly_token_supply: i64,
// }

export interface MyRewardsResponse {
  project_name: string;
  token_symbol: string;

  month: string;
  total_points: number;
  points: number;
  monthly_token_supply: number;
}

export interface PointTransactionResponse {
  month: string;
  transaction_type: TransactionType;
  amount: number;
  target_user_id?: string;
  description?: string;
  created_at: number;
}

export interface ListPointTransactionsResponse {
  items: PointTransactionResponse[];
  bookmark?: string;
}

export enum TransactionType {
  Award = 'AWARD',
  Deduct = 'DEDUCT',
  Transfer = 'TRANSFER',
  Exchange = 'EXCHANGE',
}
