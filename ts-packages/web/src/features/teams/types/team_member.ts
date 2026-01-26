import { MemberGroup } from './member_group';

export interface TeamMember {
  user_id: string;
  username: string;
  display_name: string;
  profile_url: string;
  groups: MemberGroup[];
  is_owner: boolean;
  evm_address?: string;
}
