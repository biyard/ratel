import { MembershipTier } from '../types/membership-tier';

export class MembershipResponse {
  id: string;
  tier: MembershipTier;
  price_dollars: number;
  credits: number;
  duration_days: number;
  display_order: number;
  is_active: boolean;
  created_at: number;
  updated_at: number;

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
  }

  isPaid(): boolean {
    return this.tier !== MembershipTier.Free;
  }
}
