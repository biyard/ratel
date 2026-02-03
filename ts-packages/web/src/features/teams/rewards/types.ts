export interface TeamRewardsResponse {
  project_name: string;
  token_symbol: string;
  month: string;
  total_points: number;
  team_points: number;
  monthly_token_supply: number;
}

// Reuse existing PointTransactionResponse and related types from user rewards
export type {
  PointTransactionResponse,
  ListPointTransactionsResponse,
  TransactionType,
} from '@/app/rewards/types';
