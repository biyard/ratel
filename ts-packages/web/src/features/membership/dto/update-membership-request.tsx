import { MembershipTier } from '../types/membership-tier';

export interface UpdateMembershipRequest {
  tier?: MembershipTier;
  price_dollars?: number;
  credits?: number;
  duration_days?: number;
  display_order?: number;
  is_active?: boolean;
}
