export interface PurchaseMembershipRequest {
  membership_id: string;
  payment_method: string | null | undefined;
  transaction_id: string | null | undefined;
}
