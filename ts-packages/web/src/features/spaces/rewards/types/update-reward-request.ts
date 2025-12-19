import { RewardTypeRequest } from './reward-type-request';

export interface UpdateRewardRequest {
  reward: RewardTypeRequest;
  label: string;
  description: string;
  credits: number;
}
