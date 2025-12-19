import { RewardTypeRequest } from './reward-type-request';

export interface CreateRewardRequest {
  reward: RewardTypeRequest;
  label: string;
  description: string;
  credits: number;
}
