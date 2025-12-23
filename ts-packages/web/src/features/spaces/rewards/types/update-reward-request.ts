import { RewardTypeRequest } from './reward-type-request';

export interface UpdateRewardRequest {
  reward: RewardTypeRequest;
  description?: string;
  credits: number;
}
