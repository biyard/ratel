import { MembershipTier } from '../types/membership-tier';

export interface UpdateMembershipRequest {
  tier?: MembershipTier;
  price_dollars?: number;
  credits?: number;
  duration_days?: number; // -1 or 0 for infinite duration
  display_order?: number;
  is_active?: boolean;
  max_credits_per_space?: number; // -1 for unlimited
}
