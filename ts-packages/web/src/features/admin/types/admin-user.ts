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
