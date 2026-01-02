import { SpaceRewardResponse } from './space-reward-response';

export class ListRewardsResponse {
  items: SpaceRewardResponse[];
  bookmark: string | null;

  constructor(data: Partial<ListRewardsResponse>) {
    this.items = (data.items ?? []).map(
      (d: Partial<SpaceRewardResponse>) => new SpaceRewardResponse(d),
    );
    this.bookmark = data.bookmark ?? null;
  }
}
