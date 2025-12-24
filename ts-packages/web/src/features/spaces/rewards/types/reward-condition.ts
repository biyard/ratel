export type RewardCondition =
  | { None: Record<string, never> }
  | { MaxClaims: number }
  | { MaxPoints: number }
  | { MaxUserClaims: number }
  | { MaxUserPoints: number };
