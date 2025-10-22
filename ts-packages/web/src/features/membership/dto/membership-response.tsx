import { MembershipTier } from '../types/membership-tier';

export interface MembershipResponse {
  id: string;
  tier: MembershipTier;
  price_dollers: number;
  credits: number;
  duration_days: number;
  display_order: number;
  is_active: boolean;
  created_at: number;
  updated_at: number;
}
