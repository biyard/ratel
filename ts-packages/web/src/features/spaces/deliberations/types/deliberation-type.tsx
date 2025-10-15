import { File } from '../utils/deliberation.spaces.v3';

export interface DiscussionInfo {
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  discussion_pk?: string;

  participants: DiscussionUser[];
}

export interface DiscussionUser {
  user_pk: string;
  display_name: string;
  profile_url: string;
  username: string;
}

export interface Deliberation {
  discussions: DiscussionInfo[];
  elearnings: ElearningCreateRequest;
}

export interface ElearningCreateRequest {
  files: File[];
}
