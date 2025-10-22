import { MembershipTier } from '../types/membership-tier';

export interface CreateMembershipRequest {
  tier: MembershipTier;
  price_dollers: number;
  credits: number;
  duration_days: number;
  display_order: number;
}
