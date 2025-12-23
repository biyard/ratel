import { RewardCondition } from './reward-condition';
import { RewardPeriod } from './reward-period';
import { RewardType } from './reward-type';

export class RewardConfig {
  reward_type: RewardType;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;

  constructor(data: Partial<RewardConfig>) {
    this.reward_type = data.reward_type ?? RewardType.PollRespond;
    this.point = data.point ?? 0;
    this.period = data.period ?? RewardPeriod.Once;
    this.condition = data.condition ?? { None: {} };
  }
}

export class ListAvailableRewardsResponse {
  items: RewardConfig[];

  constructor(data: Partial<ListAvailableRewardsResponse>) {
    this.items = (data.items ?? []).map(
      (d: Partial<RewardConfig>) => new RewardConfig(d),
    );
  }
}
