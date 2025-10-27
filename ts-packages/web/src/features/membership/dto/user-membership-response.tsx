import { MembershipStatus } from '../types/membership-status';

export class UserMembershipResponse {
  user_id: string;
  membership_id: string;
  status: MembershipStatus;
  total_credits: number;
  remaining_credits: number;
  auto_renew: boolean;
  renewal_count: number;
  price_paid: number;
  transaction_id: string | undefined | null;
  created_at: number;
  updated_at: number;
  expired_at: number;
  cancelled_at: number | undefined | null;
  cancellation_reason: string | undefined | null;

  constructor(json) {
    this.user_id = json.user_id;
    this.membership_id = json.membership_id;
    this.status = json.status;
    this.total_credits = json.total_credits;
    this.remaining_credits = json.remaining_credits;
    this.auto_renew = json.auto_renew;
    this.renewal_count = json.renewal_count;
    this.price_paid = json.price_paid;
    this.transaction_id = json.transaction_id;
    this.created_at = json.created_at;
    this.updated_at = json.updated_at;
    this.expired_at = json.expired_at;
    this.cancelled_at = json.cancelled_at;
    this.cancellation_reason = json.cancellation_reason;
  }
}
