export interface Team {
  pk: string;
  created_at: number;
  updated_at: number;
  nickname: string;
  username: string;
  profile_url?: string;
  user_type: number;
  html_contents: string;
  permissions?: bigint;
  dao_address?: string;
}

// export interface TeamOwnerResponse {
//   id: string; // Updated from 'user_pk' - just the UUID, not the full Partition
//   display_name: string;
//   profile_url: string;
//   username: string;
// }
