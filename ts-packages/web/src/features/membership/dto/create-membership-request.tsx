import { MembershipTier } from '../types/membership-tier';

export interface CreateMembershipRequest {
  tier: MembershipTier;
  price_dollars: number;
  credits: number;
  duration_days: number; // -1 or 0 for infinite duration
  display_order: number;
  max_credits_per_space: number; // -1 for unlimited
}
