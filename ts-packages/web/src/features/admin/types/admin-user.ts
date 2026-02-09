export enum UserType {
  Individual = 'Individual',
  Organization = 'Organization',
  Admin = 'Admin',
  ServiceAdmin = 'ServiceAdmin',
}

export interface AdminUser {
  user_id: string;
  username: string;
  email: string;
  display_name: string;
  profile_url: string;
  created_at: number;
  user_type: UserType;
}

export interface AdminListResponse {
  items: AdminUser[];
  bookmark?: string;
}

export interface PromoteToAdminRequest {
  email: string;
}

export interface DemoteAdminResponse {
  success: boolean;
  message: string;
}

export interface AdminPaymentResponse {
  payment_id: string;
  status: string;
  currency: string;
  paid_at: string | null;
  order_name: string;
  user_pk: string | null;
  user_email: string | null;
  user_name: string | null;
  total: number;
  cancelled: number | null;
}

export interface AdminCancelPaymentRequest {
  reason: string;
}

export interface AdminCancelPaymentResponse {
  status: string;
  cancellation_id: string;
  total_amount: number;
  reason: string;
  cancelled_at: string | null;
  requested_at: string;
}

export interface AdminPaymentListResponse {
  items: AdminPaymentResponse[];
  bookmark: string | null;
}
