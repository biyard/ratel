import { call } from './call';
import type {
  AdminUser,
  AdminListResponse,
  PromoteToAdminRequest,
  DemoteAdminResponse,
  AdminPaymentListResponse,
} from '@/features/admin/types/admin-user';

/**
 * List all admin users
 * GET /m3/admin
 */
export async function listAdmins(): Promise<AdminListResponse> {
  return await call('GET', '/m3/admin');
}

/**
 * Get a specific admin user by ID
 * GET /m3/admin/:user_id
 */
export async function getAdmin(userId: string): Promise<AdminUser> {
  return await call('GET', `/m3/admin/${userId}`);
}

/**
 * Promote a user to admin by email
 * POST /m3/admin
 */
export async function promoteToAdmin(email: string): Promise<AdminUser> {
  const request: PromoteToAdminRequest = { email };
  return await call('POST', '/m3/admin', request);
}

/**
 * Demote an admin to regular user
 * DELETE /m3/admin/:user_id
 */
export async function demoteAdmin(
  userId: string,
): Promise<DemoteAdminResponse> {
  return await call('DELETE', `/m3/admin/${userId}`);
}

/**
 * Run teams migration
 * POST /m3/migrations/teams
 */
export async function runTeamsMigration(): Promise<void> {
  return await call('POST', '/m3/migrations/teams');
}

/**
 * List all payments (Admin only)
 * GET /m3/payments
 */
export async function listPayments(
  page: number = 0,
): Promise<AdminPaymentListResponse> {
  const bookmark = JSON.stringify({ page });
  return call('GET', `/m3/payments?bookmark=${encodeURIComponent(bookmark)}`);
}
