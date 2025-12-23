import { RewardTypeRequest } from './reward-type-request';

export interface CreateRewardRequest {
  reward: RewardTypeRequest;
  description?: string;
  credits: number;
}
