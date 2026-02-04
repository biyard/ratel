import { RewardCondition } from './reward-condition';
import { RewardPeriod } from './reward-period';
import { RewardAction } from './reward-type';

export class SpaceRewardResponse {
  pk: string;
  sk: string;

  created_at: number;
  updated_at: number;

  reward_action: RewardAction;

  description: string;
  points: number;
  credits: number;

  total_points: number;
  total_claims: number;

  period: RewardPeriod;
  condition: RewardCondition;

  user_claims: number;

  constructor(json: Partial<SpaceRewardResponse>) {
    this.pk = json.pk ?? '';
    this.sk = json.sk ?? '';
    this.created_at = json.created_at ?? 0;
    this.updated_at = json.updated_at ?? 0;
    this.reward_action = json.reward_action;
    this.description = json.description ?? '';
    this.credits = json.credits ?? 0;
    this.total_points = json.total_points ?? 0;
    this.total_claims = json.total_claims ?? 0;
    this.period = json.period ?? RewardPeriod.Once;
    this.condition = json.condition ?? 'None';
    this.user_claims = json.user_claims ?? 0;
  }
}
