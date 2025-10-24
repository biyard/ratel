import { MembershipTier } from '../types/membership-tier';

export class MembershipResponse {
  id: string;
  tier: MembershipTier;
  price_dollars: number;
  credits: number;
  duration_days: number; // -1 or 0 for infinite duration
  display_order: number;
  is_active: boolean;
  created_at: number;
  updated_at: number;
  max_credits_per_space: number; // -1 for unlimited

  constructor(json) {
    this.id = json.id;
    this.tier = json.tier;
    this.price_dollars = json.price_dollars;
    this.credits = json.credits;
    this.duration_days = json.duration_days;
    this.display_order = json.display_order;
    this.is_active = json.is_active;
    this.created_at = json.created_at;
    this.updated_at = json.updated_at;
    this.max_credits_per_space = json.max_credits_per_space;
  }

  isPaid(): boolean {
    return this.tier !== MembershipTier.Free;
  }

  isInfiniteDuration(): boolean {
    return this.duration_days <= 0;
  }

  isUnlimitedCreditsPerSpace(): boolean {
    return this.max_credits_per_space <= 0;
  }

  getFormattedDuration(): string {
    if (this.isInfiniteDuration()) {
      return 'Unlimited';
    }
    return `${this.duration_days} days`;
  }

  getFormattedCreditsPerSpace(): string {
    if (this.isUnlimitedCreditsPerSpace()) {
      return 'Unlimited';
    }
    return this.max_credits_per_space.toLocaleString();
  }
}
