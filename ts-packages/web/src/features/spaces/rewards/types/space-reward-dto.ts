import { RewardCondition } from './reward-condition';
import { RewardPeriod } from './reward-period';
import { RewardUserBehavior } from './reward-user-behavior';

export class SpaceRewardResponse {
  pk: string;
  sk: string;

  created_at: number;
  updated_at: number;

  behavior: RewardUserBehavior;

  description: string;
  points: number;
  credits: number;

  total_points: number;
  total_claims: number;

  period: RewardPeriod;
  condition: RewardCondition;

  user_claims: number;
  user_points: number;

  constructor(json: Partial<SpaceRewardResponse>) {
    this.pk = json.pk ?? '';
    this.sk = json.sk ?? '';
    this.created_at = json.created_at ?? 0;
    this.updated_at = json.updated_at ?? 0;
    this.behavior = json.behavior ?? RewardUserBehavior.RespondPoll;
    this.description = json.description ?? '';
    this.points = json.points ?? 0;
    this.credits = json.credits ?? 0;
    this.total_points = json.total_points ?? 0;
    this.total_claims = json.total_claims ?? 0;
    this.period = json.period ?? RewardPeriod.Once;
    this.condition = json.condition ?? 'None';
    this.user_claims = json.user_claims ?? 0;
    this.user_points = json.user_points ?? 0;
  }
}

export interface CreateSpaceRewardRequest {
  action_key: string;
  behavior: RewardUserBehavior;
  description?: string;
  credits: number;
}

export interface UpdateSpaceRewardRequest {
  sk: string;
  description?: string;
  credits: number;
}

export interface DeleteSpaceRewardRequest {
  sk: string;
}
