import { RewardCondition } from './reward-condition';
import { RewardPeriod } from './reward-period';

// Global reward action types (matches backend RewardAction enum)
export type GlobalRewardAction = 'None' | 'PollRespond';

// Response from GET /v3/rewards
export interface GlobalRewardResponse {
  reward_action: GlobalRewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

// Request for PATCH /m3/rewards
export interface UpdateGlobalRewardRequest {
  action: GlobalRewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}
